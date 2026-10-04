import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.db.reference_lookup import get_options, get_record_by_id
from arlo_db_tool.domain.draft_factory import create_league_calendar_config_draft
from arlo_db_tool.generation.bulk_team_batch_generator import (
    TeamDraft,
    build_team_insert_statements,
    generate_team_batch,
)
from arlo_db_tool.generation.bulk_venue_batch_generator import (
    generate_venue_batch,
)
from arlo_db_tool.generation.manager_batch_generator import generate_manager_batch
from arlo_db_tool.generation.manager_draft import ManagerDraft
from arlo_db_tool.generation.player_batch_generator import generate_player_batch
from arlo_db_tool.generation.player_draft import PlayerDraft
from arlo_db_tool.generation.referee_batch_generator import generate_referee_batch
from arlo_db_tool.generation.referee_draft import RefereeDraft
from arlo_db_tool.generation.squad_role_distribution import (
    build_squad_quotas_for_prestige,
)
from arlo_db_tool.generation.team_prestige_ca_mapper import (
    map_prestige_to_ca_range,
)
from arlo_db_tool.generation.venue_draft import VenueDraft
from arlo_db_tool.sql.aggregate_statement_builder import gerar_statements_create
from arlo_db_tool.sql.bulk_manager_statement_builder import (
    build_bulk_manager_statements,
)
from arlo_db_tool.sql.bulk_player_statement_builder import (
    build_bulk_player_statements,
)
from arlo_db_tool.sql.bulk_referee_statement_builder import (
    build_bulk_referee_statements,
)
from arlo_db_tool.sql.bulk_venue_statement_builder import (
    build_bulk_venue_statements,
)
from arlo_db_tool.ui.bulk.bulk_generation_preview_panel import (
    BulkGenerationPreviewPanel,
)
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.widgets.range_input_widget import RangeInputWidget


class LeaguePopulationWizardScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.league_options_map: Dict[str, str] = {}
        self.country_options_map: Dict[str, Optional[str]] = {}

        self.last_generated_teams: List[TeamDraft] = []
        self.last_generated_venues: List[VenueDraft] = []
        self.last_generated_players: List[PlayerDraft] = []
        self.last_generated_managers: List[ManagerDraft] = []
        self.last_generated_referees: List[RefereeDraft] = []

        top_actions = ttk.Frame(self, padding=(8, 6))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Assistente de População de Liga Inteira (Full League Wizard)",
            font=("TkDefaultFont", 11, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reset = ttk.Button(top_actions, text="Resetar", command=self._reset_fields)
        self.btn_reset.pack(side=tk.RIGHT, padx=4)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Referências", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_generate = ttk.Button(
            top_actions,
            text="Gerar Liga Completa & SQL",
            command=self._generate_full_league,
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

        league_frame = ttk.LabelFrame(self.scroll_content, text="1. Seleção da Liga Alvo", padding=(10, 8))
        league_frame.pack(fill=tk.X, expand=True, pady=4)
        league_frame.columnconfigure(1, weight=1)
        league_frame.columnconfigure(3, weight=1)

        ttk.Label(league_frame, text="Liga (Competição):").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.league_var = tk.StringVar()
        self.combo_league = ttk.Combobox(league_frame, textvariable=self.league_var, state="readonly", width=38)
        self.combo_league.grid(row=0, column=1, sticky="ew", padx=(0, 16), pady=4)
        self.combo_league.bind("<<ComboboxSelected>>", self._on_league_selected)

        ttk.Label(league_frame, text="País da Liga:").grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.lbl_league_country = ttk.Label(league_frame, text="-", font=("TkDefaultFont", 9, "bold"))
        self.lbl_league_country.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(league_frame, text="Escopo da Liga:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.lbl_league_scope = ttk.Label(league_frame, text="-")
        self.lbl_league_scope.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(league_frame, text="Prestígio da Liga:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.lbl_league_prestige = ttk.Label(league_frame, text="-")
        self.lbl_league_prestige.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        teams_cfg_frame = ttk.LabelFrame(self.scroll_content, text="2. Parâmetros de Geração dos Times", padding=(10, 8))
        teams_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        teams_cfg_frame.columnconfigure(1, weight=1)
        teams_cfg_frame.columnconfigure(3, weight=1)

        ttk.Label(teams_cfg_frame, text="Quantidade de Times:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.team_count_var = tk.StringVar(value="16")
        self.spin_team_count = ttk.Spinbox(teams_cfg_frame, from_=2, to=64, textvariable=self.team_count_var, width=8)
        self.spin_team_count.grid(row=0, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(teams_cfg_frame, text="Faixa de Prestígio dos Times:").grid(
            row=0, column=2, sticky="w", padx=(4, 6), pady=4
        )
        self.team_prestige_widget = RangeInputWidget(
            teams_cfg_frame, min_limit=0, max_limit=1000, default_min=200, default_max=750
        )
        self.team_prestige_widget.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(teams_cfg_frame, text="Ano de Referência do Mundo:").grid(
            row=1, column=0, sticky="w", padx=(4, 6), pady=4
        )
        self.ref_year_var = tk.StringVar(value="3627")
        self.spin_ref_year = ttk.Spinbox(teams_cfg_frame, from_=1, to=9999, textvariable=self.ref_year_var, width=8)
        self.spin_ref_year.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(teams_cfg_frame, text="País dos Times (Opcional):").grid(
            row=1, column=2, sticky="w", padx=(4, 6), pady=4
        )
        self.team_country_var = tk.StringVar()
        self.combo_team_country = ttk.Combobox(
            teams_cfg_frame, textvariable=self.team_country_var, state="readonly", width=34
        )
        self.combo_team_country.grid(row=1, column=3, sticky="ew", padx=(0, 4), pady=4)

        venue_cfg_frame = ttk.LabelFrame(
            self.scroll_content, text="3. Estádios dos Times (Venues)", padding=(10, 8)
        )
        venue_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        venue_cfg_frame.columnconfigure(1, weight=1)
        venue_cfg_frame.columnconfigure(3, weight=1)

        self.gen_venues_var = tk.BooleanVar(value=True)
        self.chk_gen_venues = ttk.Checkbutton(
            venue_cfg_frame,
            text="Gerar Estádio Oficial (MatchStadium) para Cada Time da Liga",
            variable=self.gen_venues_var,
            command=self._on_toggle_venues,
        )
        self.chk_gen_venues.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 6))

        ttk.Label(venue_cfg_frame, text="Faixa de Capacidade:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.venue_cap_widget = RangeInputWidget(
            venue_cfg_frame, min_limit=5000, max_limit=150000, default_min=15000, default_max=65000, step=1000
        )
        self.venue_cap_widget.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(venue_cfg_frame, text="Comprimento Mirim (140-150):").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.venue_len_widget = RangeInputWidget(
            venue_cfg_frame, min_limit=140.0, max_limit=150.0, default_min=140.0, default_max=150.0, step=0.5, is_float=True
        )
        self.venue_len_widget.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(venue_cfg_frame, text="Largura Mirim (80-90):").grid(row=2, column=0, sticky="w", padx=(4, 6), pady=4)
        self.venue_wid_widget = RangeInputWidget(
            venue_cfg_frame, min_limit=80.0, max_limit=90.0, default_min=80.0, default_max=90.0, step=0.5, is_float=True
        )
        self.venue_wid_widget.grid(row=2, column=1, sticky="w", padx=(0, 16), pady=4)

        roster_cfg_frame = ttk.LabelFrame(
            self.scroll_content, text="4. Elencos dos Times (Jogadores)", padding=(10, 8)
        )
        roster_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        roster_cfg_frame.columnconfigure(1, weight=1)
        roster_cfg_frame.columnconfigure(3, weight=1)

        self.gen_roster_var = tk.BooleanVar(value=True)
        self.chk_gen_roster = ttk.Checkbutton(
            roster_cfg_frame,
            text="Gerar Elenco Completo de Jogadores para Cada Time (Baseado no Prestígio)",
            variable=self.gen_roster_var,
            command=self._on_toggle_roster,
        )
        self.chk_gen_roster.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 6))

        ttk.Label(roster_cfg_frame, text="Faixa de Idade Jogadores:").grid(
            row=1, column=0, sticky="w", padx=(4, 6), pady=4
        )
        self.player_age_widget = RangeInputWidget(
            roster_cfg_frame, min_limit=15, max_limit=45, default_min=18, default_max=35
        )
        self.player_age_widget.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(roster_cfg_frame, text="Viés País do Time:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.player_bias_var = tk.StringVar(value="0.70")
        self.spin_player_bias = ttk.Spinbox(
            roster_cfg_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.player_bias_var, width=8
        )
        self.spin_player_bias.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(roster_cfg_frame, text="Chance Pos. Secundárias:").grid(
            row=2, column=0, sticky="w", padx=(4, 6), pady=4
        )
        self.player_sec_chance_var = tk.StringVar(value="0.40")
        self.spin_player_sec_chance = ttk.Spinbox(
            roster_cfg_frame,
            from_=0.0,
            to=1.0,
            increment=0.05,
            textvariable=self.player_sec_chance_var,
            width=8,
        )
        self.spin_player_sec_chance.grid(row=2, column=1, sticky="w", padx=(0, 16), pady=4)

        staff_cfg_frame = ttk.LabelFrame(
            self.scroll_content, text="5. Comissão Técnica (Técnicos)", padding=(10, 8)
        )
        staff_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        staff_cfg_frame.columnconfigure(1, weight=1)
        staff_cfg_frame.columnconfigure(3, weight=1)

        self.gen_manager_var = tk.BooleanVar(value=True)
        self.chk_gen_manager = ttk.Checkbutton(
            staff_cfg_frame,
            text="Gerar Técnico (Manager) para Cada Time (CA alinhado ao Prestígio)",
            variable=self.gen_manager_var,
            command=self._on_toggle_manager,
        )
        self.chk_gen_manager.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 6))

        ttk.Label(staff_cfg_frame, text="Faixa de Idade Técnicos:").grid(
            row=1, column=0, sticky="w", padx=(4, 6), pady=4
        )
        self.manager_age_widget = RangeInputWidget(
            staff_cfg_frame, min_limit=28, max_limit=80, default_min=35, default_max=65
        )
        self.manager_age_widget.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ref_cfg_frame = ttk.LabelFrame(self.scroll_content, text="6. Arbitragem (Juízes da Liga)", padding=(10, 8))
        ref_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        ref_cfg_frame.columnconfigure(1, weight=1)
        ref_cfg_frame.columnconfigure(3, weight=1)

        self.gen_referees_var = tk.BooleanVar(value=True)
        self.chk_gen_referees = ttk.Checkbutton(
            ref_cfg_frame,
            text="Gerar Juízes / Árbitros Atribuídos a Esta Liga",
            variable=self.gen_referees_var,
            command=self._on_toggle_referees,
        )
        self.chk_gen_referees.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 6))

        ttk.Label(ref_cfg_frame, text="Quantidade de Juízes:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.ref_count_var = tk.StringVar(value="16")
        self.spin_ref_count = ttk.Spinbox(ref_cfg_frame, from_=2, to=100, textvariable=self.ref_count_var, width=8)
        self.spin_ref_count.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(ref_cfg_frame, text="Faixa de Idade Juízes:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.ref_age_widget = RangeInputWidget(
            ref_cfg_frame, min_limit=20, max_limit=70, default_min=25, default_max=55
        )
        self.ref_age_widget.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        cal_cfg_frame = ttk.LabelFrame(
            self.scroll_content, text="7. Configuração de Calendário (League Calendar Config)", padding=(10, 8)
        )
        cal_cfg_frame.pack(fill=tk.X, expand=True, pady=4)
        cal_cfg_frame.columnconfigure(1, weight=1)
        cal_cfg_frame.columnconfigure(3, weight=1)

        self.gen_calendar_var = tk.BooleanVar(value=True)
        self.chk_gen_calendar = ttk.Checkbutton(
            cal_cfg_frame,
            text="Gerar Configuração de Calendário (League Calendar Config) Oficial",
            variable=self.gen_calendar_var,
            command=self._on_toggle_calendar,
        )
        self.chk_gen_calendar.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 6))

        ttk.Label(cal_cfg_frame, text="Algoritmo de Tabela:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.cal_algo_var = tk.StringVar(value="RoundRobinDoubleLeg")
        self.combo_cal_algo = ttk.Combobox(
            cal_cfg_frame,
            textvariable=self.cal_algo_var,
            values=["RoundRobinDoubleLeg", "RoundRobinSingleLeg"],
            state="readonly",
            width=22,
        )
        self.combo_cal_algo.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(cal_cfg_frame, text="Duração em Semanas:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.cal_weeks_var = tk.StringVar(value="38")
        self.spin_cal_weeks = ttk.Spinbox(cal_cfg_frame, from_=1, to=104, textvariable=self.cal_weeks_var, width=8)
        self.spin_cal_weeks.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        self.preview_panel = BulkGenerationPreviewPanel(
            self.scroll_content, title="Prévia Geral da Liga a Gerar"
        )
        self.preview_panel.pack(fill=tk.X, expand=True, pady=4)

        sep_bottom = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep_bottom.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(side=tk.BOTTOM, fill=tk.X)

        self.reload_references()

    def reload_references(self):
        self.league_options_map.clear()
        self.country_options_map.clear()

        league_opts = get_options(
            self.app_state.database_path,
            "competitions",
            id_col="id",
            label_expr="name",
            where="kind = 'League'",
        )
        league_display = []
        for l_id, label in league_opts:
            disp = f"{label} [{l_id}]"
            league_display.append(disp)
            self.league_options_map[disp] = l_id
        self.combo_league["values"] = league_display

        if league_display:
            if self.league_var.get() not in league_display:
                self.combo_league.current(0)
            self._on_league_selected(None)
        else:
            self.league_var.set("")
            self.lbl_league_country.config(text="-")
            self.lbl_league_scope.config(text="-")
            self.lbl_league_prestige.config(text="-")

        country_opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        country_display = ["(Mesmo País da Liga / Automático)"]
        self.country_options_map["(Mesmo País da Liga / Automático)"] = None
        for c_id, label in country_opts:
            disp = f"{label} [{c_id}]"
            country_display.append(disp)
            self.country_options_map[disp] = c_id
        self.combo_team_country["values"] = country_display
        if country_display:
            if self.team_country_var.get() not in country_display:
                self.combo_team_country.current(0)

    def _on_league_selected(self, _event):
        sel = self.league_var.get()
        league_id = self.league_options_map.get(sel)
        if not league_id or not self.app_state.database_path:
            return

        comp_rec = get_record_by_id(self.app_state.database_path, "competitions", "id", league_id)
        if not comp_rec:
            return

        country_id = comp_rec.get("country_id", "-")
        scope = comp_rec.get("scope", "-")
        prestige = comp_rec.get("prestige", "-")

        self.lbl_league_country.config(text=str(country_id))
        self.lbl_league_scope.config(text=str(scope))
        self.lbl_league_prestige.config(text=str(prestige))

    def _on_toggle_venues(self):
        enabled = self.gen_venues_var.get()
        self.venue_cap_widget.set_enabled(enabled)
        self.venue_len_widget.set_enabled(enabled)
        self.venue_wid_widget.set_enabled(enabled)

    def _on_toggle_roster(self):
        enabled = self.gen_roster_var.get()
        self.player_age_widget.set_enabled(enabled)
        self.spin_player_bias.config(state="normal" if enabled else "disabled")
        self.spin_player_sec_chance.config(state="normal" if enabled else "disabled")

    def _on_toggle_manager(self):
        enabled = self.gen_manager_var.get()
        self.manager_age_widget.set_enabled(enabled)

    def _on_toggle_referees(self):
        enabled = self.gen_referees_var.get()
        self.spin_ref_count.config(state="normal" if enabled else "disabled")
        self.ref_age_widget.set_enabled(enabled)

    def _on_toggle_calendar(self):
        enabled = self.gen_calendar_var.get()
        self.combo_cal_algo.config(state="readonly" if enabled else "disabled")
        self.spin_cal_weeks.config(state="normal" if enabled else "disabled")

    def _reset_fields(self):
        if self.combo_league["values"]:
            self.combo_league.current(0)
            self._on_league_selected(None)
        if self.combo_team_country["values"]:
            self.combo_team_country.current(0)
        self.team_count_var.set("16")
        self.team_prestige_widget.set_range(200, 750)
        self.ref_year_var.set("3627")
        self.gen_venues_var.set(True)
        self.venue_cap_widget.set_range(15000, 65000)
        self.venue_len_widget.set_range(140.0, 150.0)
        self.venue_wid_widget.set_range(80.0, 90.0)
        self.gen_roster_var.set(True)
        self.player_age_widget.set_range(18, 35)
        self.player_bias_var.set("0.70")
        self.player_sec_chance_var.set("0.40")
        self.gen_manager_var.set(True)
        self.manager_age_widget.set_range(35, 65)
        self.gen_referees_var.set(True)
        self.ref_count_var.set("16")
        self.ref_age_widget.set_range(25, 55)
        self.gen_calendar_var.set(True)
        self.cal_algo_var.set("RoundRobinDoubleLeg")
        self.cal_weeks_var.set("38")
        self._on_toggle_venues()
        self._on_toggle_roster()
        self._on_toggle_manager()
        self._on_toggle_referees()
        self._on_toggle_calendar()
        self.preview_panel.clear()
        self.sql_output.set_sql("")
        self.last_generated_teams = []
        self.last_generated_venues = []
        self.last_generated_players = []
        self.last_generated_managers = []
        self.last_generated_referees = []

    def _generate_full_league(self):
        if not self.app_state.database_path:
            messagebox.showerror("Erro", "Nenhum banco de dados SQLite selecionado.")
            return

        sel_league = self.league_var.get()
        league_id = self.league_options_map.get(sel_league)
        if not league_id:
            messagebox.showerror("Erro", "Selecione uma liga válida.")
            return

        try:
            team_count = int(self.team_count_var.get().strip())
            ref_year = int(self.ref_year_var.get().strip())
            team_prestige_range = self.team_prestige_widget.get_range()
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Parâmetros numéricos de times inválidos.")
            return

        if team_count < 2:
            messagebox.showerror("Erro", "A liga precisa de no mínimo 2 times.")
            return

        comp_rec = get_record_by_id(self.app_state.database_path, "competitions", "id", league_id)
        league_country_id = comp_rec.get("country_id") if comp_rec else None
        league_scope = str(comp_rec.get("scope", "National")) if comp_rec else "National"

        sel_team_country = self.country_options_map.get(self.team_country_var.get())
        resolved_country_id = sel_team_country or (str(league_country_id) if league_country_id else None)

        country_opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        available_country_ids = [c_id for c_id, _ in country_opts]

        existing_team_names: List[str] = []
        existing_venue_names: List[str] = []
        try:
            with get_db_connection(self.app_state.database_path) as conn:
                cursor = conn.execute("SELECT name FROM teams;")
                existing_team_names = [str(r["name"]) for r in cursor.fetchall() if r["name"]]
                cursor = conn.execute("SELECT name FROM venues;")
                existing_venue_names = [str(r["name"]) for r in cursor.fetchall() if r["name"]]
        except Exception:
            existing_team_names = []
            existing_venue_names = []

        try:
            teams = generate_team_batch(
                count=team_count,
                league_id=league_id,
                country_id=resolved_country_id,
                available_country_ids=available_country_ids,
                prestige_range=(int(team_prestige_range[0]), int(team_prestige_range[1])),
                reference_year=ref_year,
                existing_team_names=existing_team_names,
            )

            all_venues: List[VenueDraft] = []
            if self.gen_venues_var.get():
                v_cap_range = self.venue_cap_widget.get_range()
                v_len_range = self.venue_len_widget.get_range()
                v_wid_range = self.venue_wid_widget.get_range()

                team_ids = [t.id for t in teams]
                team_names = [t.name for t in teams]
                team_countries = [t.country_id for t in teams]
                team_prestiges = [t.prestige for t in teams]

                all_venues = generate_venue_batch(
                    count=len(teams),
                    kind="MatchStadium",
                    owner_team_ids=team_ids,
                    owner_team_names=team_names,
                    owner_team_countries=team_countries,
                    owner_team_prestiges=team_prestiges,
                    available_country_ids=available_country_ids,
                    preferred_country_id=resolved_country_id,
                    capacity_range=(int(v_cap_range[0]), int(v_cap_range[1])),
                    pitch_length_range=(float(v_len_range[0]), float(v_len_range[1])),
                    pitch_width_range=(float(v_wid_range[0]), float(v_wid_range[1])),
                    existing_venue_names=existing_venue_names,
                )

                for t, v in zip(teams, all_venues):
                    t.home_venue_id = v.id

            self.last_generated_teams = teams
            self.last_generated_venues = all_venues

            all_players: List[PlayerDraft] = []
            if self.gen_roster_var.get():
                player_age_range = self.player_age_widget.get_range()
                p_bias = float(self.player_bias_var.get().strip())
                p_sec_chance = float(self.player_sec_chance_var.get().strip())

                for team in teams:
                    quotas = build_squad_quotas_for_prestige(team.prestige)
                    squad_size = sum(quotas.values())
                    ca_range = map_prestige_to_ca_range(team.prestige)

                    team_players = generate_player_batch(
                        count=squad_size,
                        position_quotas=quotas,
                        team_id=team.id,
                        ca_range=ca_range,
                        age_range=(int(player_age_range[0]), int(player_age_range[1])),
                        reference_year=ref_year,
                        available_country_ids=available_country_ids,
                        preferred_country_id=team.country_id,
                        preferred_country_bias=p_bias,
                        secondary_positions_chance=p_sec_chance,
                        db_path=self.app_state.database_path,
                    )
                    all_players.extend(team_players)

            self.last_generated_players = all_players

            all_managers: List[ManagerDraft] = []
            if self.gen_manager_var.get():
                mgr_age_range = self.manager_age_widget.get_range()
                for team in teams:
                    mgr_ca_range = map_prestige_to_ca_range(team.prestige)
                    team_mgrs = generate_manager_batch(
                        count=1,
                        team_id=team.id,
                        ca_range=mgr_ca_range,
                        age_range=(int(mgr_age_range[0]), int(mgr_age_range[1])),
                        reference_year=ref_year,
                        available_country_ids=available_country_ids,
                        preferred_country_id=team.country_id,
                        db_path=self.app_state.database_path,
                    )
                    all_managers.extend(team_mgrs)

            self.last_generated_managers = all_managers

            all_referees: List[RefereeDraft] = []
            if self.gen_referees_var.get():
                ref_count = int(self.ref_count_var.get().strip())
                ref_age_range = self.ref_age_widget.get_range()
                all_referees = generate_referee_batch(
                    count=ref_count,
                    primary_league_id=league_id,
                    tier=league_scope,
                    age_range=(int(ref_age_range[0]), int(ref_age_range[1])),
                    reference_year=ref_year,
                    available_country_ids=available_country_ids,
                    preferred_country_id=str(league_country_id) if league_country_id else None,
                    db_path=self.app_state.database_path,
                )

            self.last_generated_referees = all_referees

            all_statements: List[str] = []

            if all_venues:
                all_statements.extend(build_bulk_venue_statements(all_venues, update_teams_home_venue=False))

            team_stmts = build_team_insert_statements(teams)
            all_statements.extend(team_stmts)

            if all_players:
                all_statements.extend(build_bulk_player_statements(all_players))

            if all_managers:
                all_statements.extend(build_bulk_manager_statements(all_managers))

            if all_referees:
                all_statements.extend(build_bulk_referee_statements(all_referees))

            cal_generated = False
            if self.gen_calendar_var.get():
                cal_algo = self.cal_algo_var.get()
                cal_weeks = int(self.cal_weeks_var.get().strip())
                cal_draft = create_league_calendar_config_draft(competition_id=league_id)
                cal_draft.algorithm = cal_algo
                cal_draft.season_length_weeks = cal_weeks
                cal_stmts = gerar_statements_create(cal_draft)
                all_statements.extend(cal_stmts)
                cal_generated = True

            full_script_lines = ["BEGIN TRANSACTION;"] + all_statements + ["COMMIT;"]
            full_sql = "\n".join(full_script_lines)
            self.sql_output.set_sql(full_sql)

            metrics = [
                ("Liga Selecionada", sel_league),
                ("Times Gerados", str(len(teams))),
                ("Estádios / Venues Gerados", str(len(all_venues))),
                ("Total de Jogadores", str(len(all_players))),
                ("Total de Técnicos", str(len(all_managers))),
                ("Total de Juízes", str(len(all_referees))),
                ("Config. Calendário Gerada", "Sim" if cal_generated else "Não"),
                ("Total Comandos SQL Gerados", str(len(all_statements))),
            ]
            self.preview_panel.update_preview(
                f"População completa para a liga '{sel_league}' gerada com sucesso!",
                metrics,
            )
            messagebox.showinfo(
                "Sucesso",
                f"Geração em lote concluída com sucesso!\n\n• {len(teams)} times\n• {len(all_venues)} estádios\n• {len(all_players)} jogadores\n• {len(all_managers)} técnicos\n• {len(all_referees)} juízes",
            )

        except Exception as e:
            messagebox.showerror("Erro ao Gerar Liga Completa", str(e))
