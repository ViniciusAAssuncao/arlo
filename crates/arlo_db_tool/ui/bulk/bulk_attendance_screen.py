import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.attendance_reference_loader import load_league_attendance_teams
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.generation.attendance_reference_suggester import (
    calibration_sample_count,
    suggest_attendance_references,
)
from arlo_db_tool.sql.attendance_reference_statement_builder import (
    build_attendance_reference_update_statements,
)
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel


class BulkAttendanceScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.league_options_map: Dict[str, Optional[str]] = {}

        top = ttk.Frame(self, padding=(8, 6))
        top.pack(fill=tk.X)
        ttk.Label(
            top,
            text="Referências de Público por Liga",
            font=("TkDefaultFont", 11, "bold"),
        ).pack(side=tk.LEFT)
        ttk.Button(top, text="Recarregar", command=self.reload_references).pack(
            side=tk.RIGHT,
            padx=4,
        )
        ttk.Button(top, text="Sugerir & Gerar SQL", command=self._generate).pack(
            side=tk.RIGHT,
            padx=4,
        )

        params = ttk.LabelFrame(self, text="Parâmetros", padding=(10, 8))
        params.pack(fill=tk.X, padx=8, pady=(2, 6))
        params.columnconfigure(1, weight=1)

        ttk.Label(params, text="Liga:").grid(row=0, column=0, sticky="w", padx=(0, 6), pady=4)
        self.league_var = tk.StringVar()
        self.league_combo = ttk.Combobox(
            params,
            textvariable=self.league_var,
            state="readonly",
            width=42,
        )
        self.league_combo.grid(row=0, column=1, sticky="ew", padx=(0, 16), pady=4)

        ttk.Label(params, text="Fator de demanda:").grid(
            row=0,
            column=2,
            sticky="w",
            padx=(0, 6),
            pady=4,
        )
        self.factor_var = tk.StringVar(value="1.00")
        ttk.Spinbox(
            params,
            from_=0.50,
            to=1.50,
            increment=0.05,
            textvariable=self.factor_var,
            width=8,
        ).grid(row=0, column=3, sticky="w", pady=4)

        self.use_calibration_var = tk.BooleanVar(value=True)
        ttk.Checkbutton(
            params,
            text="Aprender com referências já preenchidas da liga",
            variable=self.use_calibration_var,
        ).grid(row=1, column=0, columnspan=2, sticky="w", pady=4)

        self.overwrite_var = tk.BooleanVar(value=False)
        ttk.Checkbutton(
            params,
            text="Sobrescrever times que já possuem min/max",
            variable=self.overwrite_var,
        ).grid(row=1, column=2, columnspan=2, sticky="w", pady=4)

        preview = ttk.LabelFrame(self, text="Prévia", padding=(6, 6))
        preview.pack(fill=tk.BOTH, expand=True, padx=8, pady=4)
        columns = (
            "team",
            "prestige",
            "capacity",
            "current_min",
            "current_max",
            "suggested_min",
            "suggested_max",
            "source",
        )
        self.tree = ttk.Treeview(preview, columns=columns, show="headings", height=15)
        headings = {
            "team": "Time",
            "prestige": "Prestígio",
            "capacity": "Capacidade",
            "current_min": "Min atual",
            "current_max": "Max atual",
            "suggested_min": "Min sugerido",
            "suggested_max": "Max sugerido",
            "source": "Base",
        }
        widths = {
            "team": 210,
            "prestige": 70,
            "capacity": 90,
            "current_min": 90,
            "current_max": 90,
            "suggested_min": 100,
            "suggested_max": 100,
            "source": 140,
        }
        for column in columns:
            self.tree.heading(column, text=headings[column])
            self.tree.column(column, width=widths[column], anchor="center")
        self.tree.column("team", anchor="w")
        self.tree.column("source", anchor="w")

        scrollbar = ttk.Scrollbar(preview, orient=tk.VERTICAL, command=self.tree.yview)
        self.tree.configure(yscrollcommand=scrollbar.set)
        self.tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

        self.summary_var = tk.StringVar(value="Selecione uma liga e gere as sugestões.")
        ttk.Label(self, textvariable=self.summary_var, padding=(8, 2)).pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(fill=tk.X)

        self.reload_references()

    def reload_references(self):
        current = self.league_var.get()
        self.league_options_map.clear()
        displays = []
        for league_id, name in get_options(
            self.app_state.database_path,
            "competitions",
            id_col="id",
            label_expr="name",
            where="kind = 'League'",
        ):
            display = f"{name} [{league_id}]"
            displays.append(display)
            self.league_options_map[display] = league_id

        self.league_combo["values"] = displays
        if current in displays:
            self.league_var.set(current)
        elif displays:
            self.league_var.set(displays[0])
        else:
            self.league_var.set("")
        self._clear_results()

    def _clear_results(self):
        for item in self.tree.get_children():
            self.tree.delete(item)
        self.sql_output.set_sql("")
        self.summary_var.set("Selecione uma liga e gere as sugestões.")

    def _generate(self):
        league_id = self.league_options_map.get(self.league_var.get())
        if not league_id:
            messagebox.showerror("Erro", "Selecione uma liga válida.")
            return

        try:
            factor = float(self.factor_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Fator de demanda inválido.")
            return

        if factor < 0.50 or factor > 1.50:
            messagebox.showerror("Erro", "O fator de demanda deve ficar entre 0.50 e 1.50.")
            return

        try:
            teams = load_league_attendance_teams(self.app_state.database_path, league_id)
            suggestions = suggest_attendance_references(
                teams,
                demand_factor=factor,
                use_calibration=self.use_calibration_var.get(),
            )
        except Exception as exc:
            messagebox.showerror("Erro ao carregar público", str(exc))
            return

        suggestion_by_id = {item.team_id: item for item in suggestions}
        for item in self.tree.get_children():
            self.tree.delete(item)

        for team in teams:
            suggestion = suggestion_by_id.get(team.id)
            if suggestion is None:
                values = (
                    team.name,
                    team.prestige,
                    team.capacity if team.capacity is not None else "-",
                    team.min_attendance if team.min_attendance is not None else "-",
                    team.max_attendance if team.max_attendance is not None else "-",
                    "-",
                    "-",
                    "sem capacidade",
                )
            else:
                source = "calibração da liga" if suggestion.used_calibration else "prestígio + liga"
                values = (
                    team.name,
                    team.prestige,
                    suggestion.capacity,
                    suggestion.current_min if suggestion.current_min is not None else "-",
                    suggestion.current_max if suggestion.current_max is not None else "-",
                    suggestion.suggested_min,
                    suggestion.suggested_max,
                    source,
                )
            self.tree.insert("", tk.END, values=values)

        statements = build_attendance_reference_update_statements(
            suggestions,
            overwrite_existing=self.overwrite_var.get(),
        )
        self.sql_output.set_sql("\n".join(statements))
        calibrated = calibration_sample_count(teams)
        skipped = len(teams) - len(suggestions)
        self.summary_var.set(
            f"{len(teams)} times | {len(statements)} updates | "
            f"{calibrated} referências calibradoras | {skipped} sem capacidade"
        )
