import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options, get_record_by_id
from arlo_db_tool.generation.player_draft import PlayerDraft
from arlo_db_tool.generation.squad_role_distribution import build_squad_quotas_for_prestige
from arlo_db_tool.generation.team_prestige_ca_mapper import map_prestige_to_ca_range
from arlo_db_tool.generation.team_roster_request_builder import build_team_roster_batch
from arlo_db_tool.sql.bulk_player_statement_builder import generate_bulk_player_sql
from arlo_db_tool.ui.bulk.bulk_generation_preview_panel import BulkGenerationPreviewPanel
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.widgets.range_input_widget import RangeInputWidget


class BulkTeamRosterScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.team_options_map: Dict[str, str] = {}
        self.last_generated_players: List[PlayerDraft] = []

        top_actions = ttk.Frame(self, padding=(8, 6))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Geração de Elenco Completo por Prestígio",
            font=("TkDefaultFont", 11, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Times", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_generate = ttk.Button(
            top_actions, text="Gerar Elenco Automático (SQL)", command=self._generate_roster
        )
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

        team_select_frame = ttk.LabelFrame(self.scroll_content, text="Seleção do Time", padding=(10, 8))
        team_select_frame.pack(fill=tk.X, expand=True, pady=4)
        team_select_frame.columnconfigure(1, weight=1)

        ttk.Label(team_select_frame, text="Time Alvo:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.team_var = tk.StringVar()
        self.combo_team = ttk.Combobox(team_select_frame, textvariable=self.team_var, state="readonly", width=42)
        self.combo_team.grid(row=0, column=1, sticky="ew", padx=(0, 8), pady=4)
        self.combo_team.bind("<<ComboboxSelected>>", self._on_team_selected)

        self.info_frame = ttk.LabelFrame(self.scroll_content, text="Informações Calculadas do Time", padding=(10, 8))
        self.info_frame.pack(fill=tk.X, expand=True, pady=4)
        self.info_frame.columnconfigure(1, weight=1)
        self.info_frame.columnconfigure(3, weight=1)

        ttk.Label(self.info_frame, text="Prestígio:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=2)
        self.lbl_prestige = ttk.Label(self.info_frame, text="-", font=("TkDefaultFont", 9, "bold"))
        self.lbl_prestige.grid(row=0, column=1, sticky="w", padx=(0, 16), pady=2)

        ttk.Label(self.info_frame, text="Faixa de CA Alvo:").grid(row=0, column=2, sticky="w", padx=(4, 6), pady=2)
        self.lbl_ca_range = ttk.Label(self.info_frame, text="-", font=("TkDefaultFont", 9, "bold"))
        self.lbl_ca_range.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=2)

        ttk.Label(self.info_frame, text="Tamanho do Elenco Calculado:").grid(
            row=1, column=0, sticky="w", padx=(4, 6), pady=2
        )
        self.lbl_squad_size = ttk.Label(self.info_frame, text="-", font=("TkDefaultFont", 9, "bold"))
        self.lbl_squad_size.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=2)

        ttk.Label(self.info_frame, text="País do Time:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=2)
        self.lbl_country = ttk.Label(self.info_frame, text="-")
        self.lbl_country.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=2)

        tuning_frame = ttk.LabelFrame(self.scroll_content, text="Ajustes de Geração", padding=(10, 8))
        tuning_frame.pack(fill=tk.X, expand=True, pady=4)
        tuning_frame.columnconfigure(1, weight=1)
        tuning_frame.columnconfigure(3, weight=1)

        ttk.Label(tuning_frame, text="Faixa de Idade:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.age_widget = RangeInputWidget(tuning_frame, min_limit=15, max_limit=45, default_min=18, default_max=35)
        self.age_widget.grid(row=0, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(tuning_frame, text="Ano de Referência:").grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.ref_year_var = tk.StringVar(value="3627")
        self.spin_ref_year = ttk.Spinbox(tuning_frame, from_=1, to=9999, textvariable=self.ref_year_var, width=8)
        self.spin_ref_year.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(tuning_frame, text="Viés País do Time:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.country_bias_var = tk.StringVar(value="0.70")
        self.spin_country_bias = ttk.Spinbox(
            tuning_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.country_bias_var, width=8
        )
        self.spin_country_bias.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(tuning_frame, text="Chance Pos. Secundárias:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.sec_chance_var = tk.StringVar(value="0.40")
        self.spin_sec_chance = ttk.Spinbox(
            tuning_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.sec_chance_var, width=8
        )
        self.spin_sec_chance.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        self.preview_panel = BulkGenerationPreviewPanel(self.scroll_content, title="Prévia do Elenco Gerado")
        self.preview_panel.pack(fill=tk.X, expand=True, pady=4)

        sep_bottom = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep_bottom.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(side=tk.BOTTOM, fill=tk.X)

        self.reload_references()

    def reload_references(self):
        self.team_options_map.clear()
        team_opts = get_options(self.app_state.database_path, "teams", id_col="id", label_expr="name")
        team_display = []
        for t_id, label in team_opts:
            disp = f"{label} [{t_id}]"
            team_display.append(disp)
            self.team_options_map[disp] = t_id
        self.combo_team["values"] = team_display
        if team_display:
            if self.team_var.get() not in team_display:
                self.combo_team.current(0)
            self._on_team_selected(None)
        else:
            self.team_var.set("")
            self.lbl_prestige.config(text="-")
            self.lbl_ca_range.config(text="-")
            self.lbl_squad_size.config(text="-")
            self.lbl_country.config(text="-")

    def _on_team_selected(self, _event):
        sel_display = self.team_var.get()
        team_id = self.team_options_map.get(sel_display)
        if not team_id or not self.app_state.database_path:
            return

        team_rec = get_record_by_id(self.app_state.database_path, "teams", "id", team_id)
        if not team_rec:
            return

        try:
            prestige = int(team_rec.get("prestige", 0))
        except (ValueError, TypeError):
            prestige = 0

        ca_min, ca_max = map_prestige_to_ca_range(prestige)
        quotas = build_squad_quotas_for_prestige(prestige)
        squad_size = sum(quotas.values())
        country_id = team_rec.get("country_id", "-")

        self.lbl_prestige.config(text=str(prestige))
        self.lbl_ca_range.config(text=f"{ca_min} - {ca_max}")
        self.lbl_squad_size.config(text=f"{squad_size} jogadores")
        self.lbl_country.config(text=str(country_id))

    def _generate_roster(self):
        if not self.app_state.database_path:
            messagebox.showerror("Erro", "Nenhum banco de dados SQLite selecionado.")
            return

        sel_display = self.team_var.get()
        team_id = self.team_options_map.get(sel_display)
        if not team_id:
            messagebox.showerror("Erro", "Selecione um time válido.")
            return

        age_range = self.age_widget.get_range()
        try:
            bias = float(self.country_bias_var.get().strip())
            sec_chance = float(self.sec_chance_var.get().strip())
            ref_year = int(self.ref_year_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Verifique os valores numéricos dos parâmetros.")
            return

        try:
            players = build_team_roster_batch(
                team_id=team_id,
                db_path=self.app_state.database_path,
                age_range=(int(age_range[0]), int(age_range[1])),
                reference_year=ref_year,
                preferred_country_bias=bias,
                secondary_positions_chance=sec_chance,
            )
            self.last_generated_players = players
            sql_text = generate_bulk_player_sql(players)
            self.sql_output.set_sql(sql_text)

            total_positions = sum(len(p.positions) for p in players)
            total_attributes = sum(len(p.attributes) for p in players)

            metrics = [
                ("Time Alvo", sel_display),
                ("Total de Jogadores Gerados", str(len(players))),
                ("Prestígio do Time", self.lbl_prestige.cget("text")),
                ("Faixa de CA Aplicada", self.lbl_ca_range.cget("text")),
                ("Total Vínculos Posições", str(total_positions)),
                ("Total Registros Atributos", str(total_attributes)),
            ]
            self.preview_panel.update_preview(
                f"Elenco completo com {len(players)} jogadores montado com sucesso!",
                metrics,
            )
            messagebox.showinfo("Sucesso", f"Elenco completo gerado com sucesso ({len(players)} jogadores)!")
        except Exception as e:
            messagebox.showerror("Erro ao Gerar Elenco", str(e))