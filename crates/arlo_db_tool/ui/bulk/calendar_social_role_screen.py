import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.calendar_social_role_loader import load_calendar_social_days
from arlo_db_tool.db.reference_lookup import get_calendar_options
from arlo_db_tool.domain.calendar_social_role_draft import CalendarSocialDay
from arlo_db_tool.sql.calendar_social_role_statement_builder import (
    build_calendar_social_role_updates,
)
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel

NULL_LABEL = "(Neutro / NULL)"
ROLE_CHOICES = [NULL_LABEL, "workday", "rest_day"]


class CalendarSocialRoleScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.calendar_options_map: Dict[str, str] = {}
        self.days: List[CalendarSocialDay] = []
        self.role_vars: Dict[str, tk.StringVar] = {}

        top = ttk.Frame(self, padding=(8, 6))
        top.pack(fill=tk.X)
        ttk.Label(
            top,
            text="Função Social dos Dias do Calendário",
            font=("TkDefaultFont", 11, "bold"),
        ).pack(side=tk.LEFT)
        ttk.Button(top, text="Recarregar", command=self.reload_references).pack(
            side=tk.RIGHT,
            padx=4,
        )
        ttk.Button(top, text="Gerar SQL", command=self._generate_sql).pack(
            side=tk.RIGHT,
            padx=4,
        )

        params = ttk.LabelFrame(self, text="Calendário e preenchimento", padding=(10, 8))
        params.pack(fill=tk.X, padx=8, pady=(2, 6))
        params.columnconfigure(1, weight=1)

        ttk.Label(params, text="Calendário:").grid(row=0, column=0, sticky="w", padx=(0, 6), pady=4)
        self.calendar_var = tk.StringVar()
        self.calendar_combo = ttk.Combobox(
            params,
            textvariable=self.calendar_var,
            state="readonly",
            width=42,
        )
        self.calendar_combo.grid(row=0, column=1, sticky="ew", padx=(0, 12), pady=4)
        self.calendar_combo.bind("<<ComboboxSelected>>", self._on_calendar_changed)

        ttk.Label(params, text="Dias de descanso no fim do ciclo:").grid(
            row=0,
            column=2,
            sticky="w",
            padx=(0, 6),
            pady=4,
        )
        self.rest_count_var = tk.StringVar(value="2")
        ttk.Spinbox(
            params,
            from_=0,
            to=20,
            textvariable=self.rest_count_var,
            width=6,
        ).grid(row=0, column=3, sticky="w", pady=4)

        actions = ttk.Frame(params)
        actions.grid(row=1, column=0, columnspan=4, sticky="w", pady=(4, 0))
        ttk.Button(actions, text="Aplicar padrão do ciclo", command=self._apply_cycle_pattern).pack(
            side=tk.LEFT,
            padx=(0, 4),
        )
        ttk.Button(actions, text="Todos workday", command=lambda: self._set_all("workday")).pack(
            side=tk.LEFT,
            padx=4,
        )
        ttk.Button(actions, text="Todos rest_day", command=lambda: self._set_all("rest_day")).pack(
            side=tk.LEFT,
            padx=4,
        )
        ttk.Button(actions, text="Limpar", command=lambda: self._set_all(None)).pack(
            side=tk.LEFT,
            padx=4,
        )

        self.days_frame = ttk.LabelFrame(self, text="Dias", padding=(8, 8))
        self.days_frame.pack(fill=tk.BOTH, expand=True, padx=8, pady=4)
        self.days_frame.columnconfigure(1, weight=1)

        self.summary_var = tk.StringVar(value="Nenhum calendário carregado.")
        ttk.Label(self, textvariable=self.summary_var, padding=(8, 2)).pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(fill=tk.X)

        self.reload_references()

    def reload_references(self):
        current = self.calendar_var.get()
        self.calendar_options_map.clear()
        displays = []
        for calendar_id, name in get_calendar_options(self.app_state.database_path):
            display = f"{name} [{calendar_id}]"
            displays.append(display)
            self.calendar_options_map[display] = calendar_id

        self.calendar_combo["values"] = displays
        if current in displays:
            self.calendar_var.set(current)
        elif displays:
            self.calendar_var.set(displays[0])
        else:
            self.calendar_var.set("")
        self._load_selected_calendar()

    def _on_calendar_changed(self, _event):
        self._load_selected_calendar()

    def _load_selected_calendar(self):
        calendar_id = self.calendar_options_map.get(self.calendar_var.get())
        if not calendar_id:
            self.days = []
            self._render_days()
            return

        try:
            self.days = load_calendar_social_days(
                self.app_state.database_path,
                calendar_id,
            )
        except Exception as exc:
            messagebox.showerror("Erro ao carregar dias", str(exc))
            self.days = []
        self._render_days()

    def _render_days(self):
        for child in self.days_frame.winfo_children():
            child.destroy()
        self.role_vars.clear()

        headers = ["Índice", "Nome", "Atual", "Nova função social"]
        for column, label in enumerate(headers):
            ttk.Label(
                self.days_frame,
                text=label,
                font=("TkDefaultFont", 9, "bold"),
            ).grid(row=0, column=column, sticky="w", padx=6, pady=(0, 4))

        for row_index, day in enumerate(self.days, start=1):
            ttk.Label(self.days_frame, text=str(day.order_index)).grid(
                row=row_index,
                column=0,
                sticky="w",
                padx=6,
                pady=3,
            )
            ttk.Label(self.days_frame, text=day.name).grid(
                row=row_index,
                column=1,
                sticky="w",
                padx=6,
                pady=3,
            )
            ttk.Label(self.days_frame, text=day.social_role or "neutro").grid(
                row=row_index,
                column=2,
                sticky="w",
                padx=6,
                pady=3,
            )
            value = day.social_role if day.social_role is not None else NULL_LABEL
            var = tk.StringVar(value=value)
            self.role_vars[day.id] = var
            ttk.Combobox(
                self.days_frame,
                textvariable=var,
                values=ROLE_CHOICES,
                state="readonly",
                width=20,
            ).grid(row=row_index, column=3, sticky="w", padx=6, pady=3)

        self.sql_output.set_sql("")
        self.summary_var.set(f"{len(self.days)} dias carregados.")

    def _set_all(self, role: Optional[str]):
        value = role if role is not None else NULL_LABEL
        for var in self.role_vars.values():
            var.set(value)

    def _apply_cycle_pattern(self):
        try:
            rest_count = int(self.rest_count_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Quantidade de dias de descanso inválida.")
            return

        rest_count = max(0, min(rest_count, len(self.days)))
        split_index = len(self.days) - rest_count
        for index, day in enumerate(self.days):
            self.role_vars[day.id].set("workday" if index < split_index else "rest_day")

    def _generate_sql(self):
        roles: Dict[str, Optional[str]] = {}
        for day_id, var in self.role_vars.items():
            value = var.get()
            roles[day_id] = None if value == NULL_LABEL else value

        statements = build_calendar_social_role_updates(self.days, roles)
        self.sql_output.set_sql("\n".join(statements))
        self.summary_var.set(
            f"{len(self.days)} dias carregados | {len(statements)} alterações"
        )
