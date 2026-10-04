import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.generation.manager_batch_generator import generate_manager_batch
from arlo_db_tool.generation.manager_draft import ManagerDraft
from arlo_db_tool.schema.entities.manager import MANAGER_CONTROL_MODES
from arlo_db_tool.sql.bulk_manager_statement_builder import generate_bulk_manager_sql
from arlo_db_tool.ui.bulk.bulk_generation_preview_panel import BulkGenerationPreviewPanel
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.widgets.range_input_widget import RangeInputWidget


class BulkManagerScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.team_options_map: Dict[str, Optional[str]] = {}
        self.country_options_map: Dict[str, Optional[str]] = {}
        self.last_generated_managers: List[ManagerDraft] = []

        top_actions = ttk.Frame(self, padding=(8, 6))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Geração em Massa de Técnicos (Managers)",
            font=("TkDefaultFont", 11, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reset = ttk.Button(top_actions, text="Resetar", command=self._reset_fields)
        self.btn_reset.pack(side=tk.RIGHT, padx=4)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Referências", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_generate = ttk.Button(top_actions, text="Gerar Técnicos & SQL", command=self._generate)
        self.btn_generate.pack(side=tk.RIGHT, padx=4)

        sep0 = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep0.pack(fill=tk.X)

        self.canvas = tk.Canvas(self, borderwidth=0, highlightthickness=0)
        self.scrollbar = ttk.Scrollbar(self, orient=tk.VERTICAL, command=self.canvas.yview)
        self.scroll_content = ttk.Frame(self.canvas, padding=(8, 8))

        self.scroll_content.bind(
            "<Configure>",
            lambda _e: self.canvas.configure(scrollregion=self.canvas.bbox("all")),
        )
        self.canvas_frame = self.canvas.create_window((0, 0), window=self.scroll_content, anchor="nw")
        self.canvas.configure(xscrollcommand=None, yscrollcommand=self.scrollbar.set)

        self.canvas.bind(
            "<Configure>",
            lambda e: self.canvas.itemconfig(self.canvas_frame, width=e.width),
        )

        self.canvas.pack(side=tk.TOP, fill=tk.BOTH, expand=True)
        self.scrollbar.pack(side=tk.RIGHT, fill=tk.Y, before=self.canvas)

        params_frame = ttk.LabelFrame(self.scroll_content, text="Parâmetros de Geração de Técnicos", padding=(10, 8))
        params_frame.pack(fill=tk.X, expand=True, pady=4)
        params_frame.columnconfigure(1, weight=1)
        params_frame.columnconfigure(3, weight=1)

        ttk.Label(params_frame, text="Time de Destino:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.team_var = tk.StringVar()
        self.combo_team = ttk.Combobox(params_frame, textvariable=self.team_var, state="readonly", width=34)
        self.combo_team.grid(row=0, column=1, sticky="ew", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Quantidade de Técnicos:").grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.count_var = tk.StringVar(value="10")
        self.spin_count = ttk.Spinbox(params_frame, from_=1, to=200, textvariable=self.count_var, width=8)
        self.spin_count.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="Faixa de CA (1-200):").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.ca_widget = RangeInputWidget(params_frame, min_limit=1, max_limit=200, default_min=80, default_max=140)
        self.ca_widget.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Faixa de Idade:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.age_widget = RangeInputWidget(params_frame, min_limit=28, max_limit=80, default_min=35, default_max=65)
        self.age_widget.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="País Preferencial:").grid(row=2, column=0, sticky="w", padx=(4, 6), pady=4)
        self.country_var = tk.StringVar()
        self.combo_country = ttk.Combobox(params_frame, textvariable=self.country_var, state="readonly", width=34)
        self.combo_country.grid(row=2, column=1, sticky="ew", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Viés País Preferencial:").grid(row=2, column=2, sticky="w", padx=(4, 6), pady=4)
        self.country_bias_var = tk.StringVar(value="0.70")
        self.spin_country_bias = ttk.Spinbox(
            params_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.country_bias_var, width=8
        )
        self.spin_country_bias.grid(row=2, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="Modo de Controle:").grid(row=3, column=0, sticky="w", padx=(4, 6), pady=4)
        self.control_mode_var = tk.StringVar(value="Ai")
        self.combo_control_mode = ttk.Combobox(
            params_frame, textvariable=self.control_mode_var, values=MANAGER_CONTROL_MODES, state="readonly", width=14
        )
        self.combo_control_mode.grid(row=3, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Ano de Referência:").grid(row=3, column=2, sticky="w", padx=(4, 6), pady=4)
        self.ref_year_var = tk.StringVar(value="3627")
        self.spin_ref_year = ttk.Spinbox(params_frame, from_=1, to=9999, textvariable=self.ref_year_var, width=8)
        self.spin_ref_year.grid(row=3, column=3, sticky="w", padx=(0, 4), pady=4)

        self.preview_panel = BulkGenerationPreviewPanel(self.scroll_content, title="Prévia do Lote de Técnicos")
        self.preview_panel.pack(fill=tk.X, expand=True, pady=4)

        sep_bottom = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep_bottom.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(side=tk.BOTTOM, fill=tk.X)

        self.reload_references()

    def reload_references(self):
        self.team_options_map.clear()
        self.country_options_map.clear()

        team_opts = get_options(self.app_state.database_path, "teams", id_col="id", label_expr="name")
        team_display = ["(Nenhum / Sem Time)"]
        self.team_options_map["(Nenhum / Sem Time)"] = None
        for t_id, label in team_opts:
            disp = f"{label} [{t_id}]"
            team_display.append(disp)
            self.team_options_map[disp] = t_id
        self.combo_team["values"] = team_display
        if team_display:
            if self.team_var.get() not in team_display:
                self.team_var.set(team_display[0])

        country_opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        country_display = ["(Aleatório / Sem Preferência)"]
        self.country_options_map["(Aleatório / Sem Preferência)"] = None
        for c_id, label in country_opts:
            disp = f"{label} [{c_id}]"
            country_display.append(disp)
            self.country_options_map[disp] = c_id
        self.combo_country["values"] = country_display
        if country_display:
            if self.country_var.get() not in country_display:
                self.country_var.set(country_display[0])

    def _reset_fields(self):
        if self.combo_team["values"]:
            self.combo_team.current(0)
        if self.combo_country["values"]:
            self.combo_country.current(0)
        self.count_var.set("10")
        self.ca_widget.set_range(80, 140)
        self.age_widget.set_range(35, 65)
        self.control_mode_var.set("Ai")
        self.country_bias_var.set("0.70")
        self.ref_year_var.set("3627")
        self.preview_panel.clear()
        self.sql_output.set_sql("")
        self.last_generated_managers = []

    def _generate(self):
        try:
            count = int(self.count_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Quantidade de técnicos inválida.")
            return

        if count <= 0:
            messagebox.showerror("Erro", "Quantidade de técnicos deve ser maior que zero.")
            return

        ca_range = self.ca_widget.get_range()
        age_range = self.age_widget.get_range()

        try:
            bias = float(self.country_bias_var.get().strip())
            ref_year = int(self.ref_year_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Verifique os valores numéricos dos parâmetros.")
            return

        team_id = self.team_options_map.get(self.team_var.get())
        pref_country_id = self.country_options_map.get(self.country_var.get())
        control_mode = self.control_mode_var.get()

        country_opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        available_country_ids = [c_id for c_id, _ in country_opts]

        try:
            managers = generate_manager_batch(
                count=count,
                team_id=team_id,
                ca_range=(int(ca_range[0]), int(ca_range[1])),
                age_range=(int(age_range[0]), int(age_range[1])),
                reference_year=ref_year,
                available_country_ids=available_country_ids,
                preferred_country_id=pref_country_id,
                preferred_country_bias=bias,
                control_mode=control_mode,
                db_path=self.app_state.database_path,
            )
            self.last_generated_managers = managers
            sql_text = generate_bulk_manager_sql(managers)
            self.sql_output.set_sql(sql_text)

            total_persons = len(managers)
            total_attributes = sum(len(m.attributes) for m in managers)

            metrics = [
                ("Total de Técnicos", str(len(managers))),
                ("Time Alvo", self.team_var.get()),
                ("Faixa de CA", f"{ca_range[0]} - {ca_range[1]}"),
                ("Faixa de Idade", f"{age_range[0]} - {age_range[1]} anos"),
                ("Registros em 'persons'", str(total_persons)),
                ("Registros em 'managers'", str(total_persons)),
                ("Total Registros Atributos", str(total_attributes)),
            ]
            self.preview_panel.update_preview(
                f"Lote com {len(managers)} técnicos gerado com sucesso!",
                metrics,
            )
        except Exception as e:
            messagebox.showerror("Erro ao Gerar Técnicos", str(e))