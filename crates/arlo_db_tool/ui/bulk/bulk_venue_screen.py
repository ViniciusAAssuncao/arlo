import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.db.reference_lookup import get_options, get_record_by_id
from arlo_db_tool.generation.bulk_venue_batch_generator import generate_venue_batch
from arlo_db_tool.generation.venue_draft import VenueDraft
from arlo_db_tool.schema.entities.venue import VENUE_KINDS
from arlo_db_tool.sql.bulk_venue_statement_builder import generate_bulk_venue_sql
from arlo_db_tool.ui.bulk.bulk_generation_preview_panel import (
    BulkGenerationPreviewPanel,
)
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.widgets.range_input_widget import RangeInputWidget

VENUE_GEN_MODES = [
    "Estádios Avulsos (Sem Time ou Time Único)",
    "Um Estádio para Cada Time de uma Liga",
    "Centros de Treinamento (Training Centers)",
]


class BulkVenueScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.league_options_map: Dict[str, Optional[str]] = {}
        self.team_options_map: Dict[str, Optional[str]] = {}
        self.country_options_map: Dict[str, Optional[str]] = {}
        self.last_generated_venues: List[VenueDraft] = []

        top_actions = ttk.Frame(self, padding=(8, 6))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Geração em Massa de Estádios & Arenas (Venues)",
            font=("TkDefaultFont", 11, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reset = ttk.Button(top_actions, text="Resetar", command=self._reset_fields)
        self.btn_reset.pack(side=tk.RIGHT, padx=4)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Referências", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_generate = ttk.Button(top_actions, text="Gerar Estádios & SQL", command=self._generate)
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

        mode_frame = ttk.LabelFrame(self.scroll_content, text="Modo de Geração de Estádios", padding=(10, 8))
        mode_frame.pack(fill=tk.X, expand=True, pady=4)
        mode_frame.columnconfigure(1, weight=1)

        ttk.Label(mode_frame, text="Alvo / Modo:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.mode_var = tk.StringVar(value=VENUE_GEN_MODES[0])
        self.combo_mode = ttk.Combobox(
            mode_frame,
            textvariable=self.mode_var,
            values=VENUE_GEN_MODES,
            state="readonly",
            width=46,
        )
        self.combo_mode.grid(row=0, column=1, sticky="w", padx=(0, 16), pady=4)
        self.combo_mode.bind("<<ComboboxSelected>>", self._on_mode_change)

        params_frame = ttk.LabelFrame(self.scroll_content, text="Parâmetros de Geração", padding=(10, 8))
        params_frame.pack(fill=tk.X, expand=True, pady=4)
        params_frame.columnconfigure(1, weight=1)
        params_frame.columnconfigure(3, weight=1)

        self.lbl_league = ttk.Label(params_frame, text="Liga Alvo:")
        self.lbl_league.grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.league_var = tk.StringVar()
        self.combo_league = ttk.Combobox(params_frame, textvariable=self.league_var, state="readonly", width=34)
        self.combo_league.grid(row=0, column=1, sticky="ew", padx=(0, 16), pady=4)

        self.lbl_team = ttk.Label(params_frame, text="Time Proprietário:")
        self.lbl_team.grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.team_var = tk.StringVar()
        self.combo_team = ttk.Combobox(params_frame, textvariable=self.team_var, state="readonly", width=34)
        self.combo_team.grid(row=0, column=3, sticky="ew", padx=(0, 4), pady=4)

        self.lbl_count = ttk.Label(params_frame, text="Quantidade de Estádios:")
        self.lbl_count.grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.count_var = tk.StringVar(value="10")
        self.spin_count = ttk.Spinbox(params_frame, from_=1, to=200, textvariable=self.count_var, width=8)
        self.spin_count.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Tipo do Venue:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.kind_var = tk.StringVar(value="MatchStadium")
        self.combo_kind = ttk.Combobox(
            params_frame,
            textvariable=self.kind_var,
            values=VENUE_KINDS,
            state="readonly",
            width=18,
        )
        self.combo_kind.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)
        self.combo_kind.bind("<<ComboboxSelected>>", self._on_kind_change)

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

        geom_frame = ttk.LabelFrame(self.scroll_content, text="Capacidade e Geometria do Campo (Mirim)", padding=(10, 8))
        geom_frame.pack(fill=tk.X, expand=True, pady=4)
        geom_frame.columnconfigure(1, weight=1)
        geom_frame.columnconfigure(3, weight=1)

        self.lbl_cap = ttk.Label(geom_frame, text="Faixa de Capacidade:")
        self.lbl_cap.grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.capacity_widget = RangeInputWidget(
            geom_frame,
            min_limit=500,
            max_limit=150000,
            default_min=15000,
            default_max=65000,
            step=1000,
        )
        self.capacity_widget.grid(row=0, column=1, sticky="w", padx=(0, 16), pady=4)

        self.lbl_link_team = ttk.Label(geom_frame, text="Vínculo de Estádio:")
        self.lbl_link_team.grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.link_home_venue_var = tk.BooleanVar(value=True)
        self.chk_link_home_venue = ttk.Checkbutton(
            geom_frame,
            text="Definir como estádio principal do time (home_venue_id)",
            variable=self.link_home_venue_var,
        )
        self.chk_link_home_venue.grid(row=0, column=3, sticky="w", padx=(0, 4), pady=4)

        self.lbl_len = ttk.Label(geom_frame, text="Comprimento Mirim (140-150):")
        self.lbl_len.grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.pitch_length_widget = RangeInputWidget(
            geom_frame,
            min_limit=140.0,
            max_limit=150.0,
            default_min=140.0,
            default_max=150.0,
            step=0.5,
            is_float=True,
        )
        self.pitch_length_widget.grid(row=1, column=1, sticky="w", padx=(0, 16), pady=4)

        self.lbl_wid = ttk.Label(geom_frame, text="Largura Mirim (80-90):")
        self.lbl_wid.grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.pitch_width_widget = RangeInputWidget(
            geom_frame,
            min_limit=80.0,
            max_limit=90.0,
            default_min=80.0,
            default_max=90.0,
            step=0.5,
            is_float=True,
        )
        self.pitch_width_widget.grid(row=1, column=3, sticky="w", padx=(0, 4), pady=4)

        self.preview_panel = BulkGenerationPreviewPanel(self.scroll_content, title="Prévia do Lote de Estádios")
        self.preview_panel.pack(fill=tk.X, expand=True, pady=4)

        sep_bottom = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep_bottom.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(side=tk.BOTTOM, fill=tk.X)

        self._on_mode_change(None)
        self.reload_references()

    def _on_mode_change(self, _event):
        mode = self.mode_var.get()
        if mode == "Um Estádio para Cada Time de uma Liga":
            self.combo_league.config(state="readonly")
            self.lbl_league.config(state="normal")
            self.combo_team.config(state="disabled")
            self.lbl_team.config(state="disabled")
            self.spin_count.config(state="disabled")
            self.lbl_count.config(state="disabled")
            self.kind_var.set("MatchStadium")
            self.combo_kind.config(state="disabled")
            self.chk_link_home_venue.config(state="normal")
            self.capacity_widget.set_enabled(True)
            self.pitch_length_widget.set_enabled(True)
            self.pitch_width_widget.set_enabled(True)
        elif mode == "Centros de Treinamento (Training Centers)":
            self.combo_league.config(state="disabled")
            self.lbl_league.config(state="disabled")
            self.combo_team.config(state="readonly")
            self.lbl_team.config(state="normal")
            self.spin_count.config(state="normal")
            self.lbl_count.config(state="normal")
            self.kind_var.set("TrainingCenter")
            self.combo_kind.config(state="disabled")
            self.chk_link_home_venue.config(state="disabled")
            self.capacity_widget.set_enabled(False)
            self.pitch_length_widget.set_enabled(False)
            self.pitch_width_widget.set_enabled(False)
        else:
            self.combo_league.config(state="disabled")
            self.lbl_league.config(state="disabled")
            self.combo_team.config(state="readonly")
            self.lbl_team.config(state="normal")
            self.spin_count.config(state="normal")
            self.lbl_count.config(state="normal")
            self.combo_kind.config(state="readonly")
            self._on_kind_change(None)

    def _on_kind_change(self, _event):
        kind = self.kind_var.get()
        is_stadium = kind == "MatchStadium"
        self.capacity_widget.set_enabled(is_stadium)
        self.pitch_length_widget.set_enabled(is_stadium)
        self.pitch_width_widget.set_enabled(is_stadium)
        self.chk_link_home_venue.config(state="normal" if is_stadium else "disabled")

    def reload_references(self):
        self.league_options_map.clear()
        self.team_options_map.clear()
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
        else:
            self.league_var.set("")

        team_opts = get_options(
            self.app_state.database_path, "teams", id_col="id", label_expr="name"
        )
        team_display = ["(Nenhum / Sem Time Proprietário)"]
        self.team_options_map["(Nenhum / Sem Time Proprietário)"] = None
        for t_id, label in team_opts:
            disp = f"{label} [{t_id}]"
            team_display.append(disp)
            self.team_options_map[disp] = t_id
        self.combo_team["values"] = team_display
        if team_display:
            if self.team_var.get() not in team_display:
                self.combo_team.current(0)

        country_opts = get_options(
            self.app_state.database_path, "countries", id_col="id", label_expr="name"
        )
        country_display = ["(Aleatório / Sem Preferência)"]
        self.country_options_map["(Aleatório / Sem Preferência)"] = None
        for c_id, label in country_opts:
            disp = f"{label} [{c_id}]"
            country_display.append(disp)
            self.country_options_map[disp] = c_id
        self.combo_country["values"] = country_display
        if country_display:
            if self.country_var.get() not in country_display:
                self.combo_country.current(0)

    def _reset_fields(self):
        self.mode_var.set(VENUE_GEN_MODES[0])
        if self.combo_league["values"]:
            self.combo_league.current(0)
        if self.combo_team["values"]:
            self.combo_team.current(0)
        if self.combo_country["values"]:
            self.combo_country.current(0)
        self.count_var.set("10")
        self.kind_var.set("MatchStadium")
        self.country_bias_var.set("0.70")
        self.capacity_widget.set_range(15000, 65000)
        self.pitch_length_widget.set_range(140.0, 150.0)
        self.pitch_width_widget.set_range(80.0, 90.0)
        self.link_home_venue_var.set(True)
        self._on_mode_change(None)
        self.preview_panel.clear()
        self.sql_output.set_sql("")
        self.last_generated_venues = []

    def _generate(self):
        if not self.app_state.database_path:
            messagebox.showerror("Erro", "Nenhum banco de dados SQLite selecionado.")
            return

        mode = self.mode_var.get()
        kind = self.kind_var.get()
        country_opts = get_options(
            self.app_state.database_path, "countries", id_col="id", label_expr="name"
        )
        available_country_ids = [c_id for c_id, _ in country_opts]
        pref_country_id = self.country_options_map.get(self.country_var.get())

        try:
            bias = float(self.country_bias_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Valor de viés inválido.")
            return

        cap_range = self.capacity_widget.get_range()
        len_range = self.pitch_length_widget.get_range()
        wid_range = self.pitch_width_widget.get_range()

        existing_names: List[str] = []
        try:
            with get_db_connection(self.app_state.database_path) as conn:
                cursor = conn.execute("SELECT name FROM venues;")
                existing_names = [str(r["name"]) for r in cursor.fetchall() if r["name"]]
        except Exception:
            existing_names = []

        try:
            if mode == "Um Estádio para Cada Time de uma Liga":
                league_id = self.league_options_map.get(self.league_var.get())
                if not league_id:
                    messagebox.showerror("Erro", "Selecione uma liga válida.")
                    return

                with get_db_connection(self.app_state.database_path) as conn:
                    cursor = conn.execute(
                        "SELECT id, name, country_id, prestige FROM teams WHERE league_id = ? ORDER BY name ASC;",
                        (str(league_id),),
                    )
                    teams = cursor.fetchall()

                if not teams:
                    messagebox.showerror("Erro", "Nenhum time cadastrado nesta liga para gerar estádios.")
                    return

                team_ids = [str(t["id"]) for t in teams]
                team_names = [str(t["name"]) for t in teams]
                team_countries = [str(t["country_id"]) if t["country_id"] else None for t in teams]
                team_prestiges = [int(t["prestige"]) if t["prestige"] is not None else 500 for t in teams]

                venues = generate_venue_batch(
                    count=len(teams),
                    kind="MatchStadium",
                    owner_team_ids=team_ids,
                    owner_team_names=team_names,
                    owner_team_countries=team_countries,
                    owner_team_prestiges=team_prestiges,
                    available_country_ids=available_country_ids,
                    preferred_country_id=pref_country_id,
                    preferred_country_bias=bias,
                    capacity_range=(int(cap_range[0]), int(cap_range[1])),
                    pitch_length_range=(float(len_range[0]), float(len_range[1])),
                    pitch_width_range=(float(wid_range[0]), float(wid_range[1])),
                    existing_venue_names=existing_names,
                )
                link_team = self.link_home_venue_var.get()
            else:
                try:
                    count = int(self.count_var.get().strip())
                except (ValueError, TypeError):
                    messagebox.showerror("Erro", "Quantidade de estádios inválida.")
                    return

                if count <= 0:
                    messagebox.showerror("Erro", "A quantidade de estádios deve ser maior que zero.")
                    return

                team_id = self.team_options_map.get(self.team_var.get())
                team_name = None
                team_country = None
                team_prestige = None

                if team_id:
                    team_rec = get_record_by_id(self.app_state.database_path, "teams", "id", team_id)
                    if team_rec:
                        team_name = str(team_rec.get("name", ""))
                        team_country = str(team_rec.get("country_id", ""))
                        team_prestige = int(team_rec.get("prestige", 500))

                venues = generate_venue_batch(
                    count=count,
                    kind=kind,
                    owner_team_id=team_id,
                    owner_team_names=[team_name] if team_name else None,
                    owner_team_countries=[team_country] if team_country else None,
                    owner_team_prestiges=[team_prestige] if team_prestige is not None else None,
                    available_country_ids=available_country_ids,
                    preferred_country_id=pref_country_id or team_country,
                    preferred_country_bias=bias,
                    capacity_range=(int(cap_range[0]), int(cap_range[1])),
                    pitch_length_range=(float(len_range[0]), float(len_range[1])),
                    pitch_width_range=(float(wid_range[0]), float(wid_range[1])),
                    existing_venue_names=existing_names,
                )
                link_team = self.link_home_venue_var.get() if (team_id and kind == "MatchStadium") else False

            self.last_generated_venues = venues
            sql_text = generate_bulk_venue_sql(venues, update_teams_home_venue=link_team)
            self.sql_output.set_sql(sql_text)

            metrics = [
                ("Total de Venues", str(len(venues))),
                ("Tipo", kind),
                ("Modo", mode),
                ("Faixa de Capacidade", f"{cap_range[0]:,} - {cap_range[1]:,}" if kind == "MatchStadium" else "N/A"),
                ("Comprimento Mirim", f"{len_range[0]:.1f} - {len_range[1]:.1f}" if kind == "MatchStadium" else "N/A"),
                ("Largura Mirim", f"{wid_range[0]:.1f} - {wid_range[1]:.1f}" if kind == "MatchStadium" else "N/A"),
                ("Vincular como home_venue_id", "Sim" if link_team else "Não"),
            ]
            self.preview_panel.update_preview(
                f"Lote com {len(venues)} venues ({kind}) gerado com sucesso!",
                metrics,
            )
            messagebox.showinfo("Sucesso", f"SQL de geração em massa para {len(venues)} venues pronto!")

        except Exception as e:
            messagebox.showerror("Erro ao Gerar Estádios", str(e))
