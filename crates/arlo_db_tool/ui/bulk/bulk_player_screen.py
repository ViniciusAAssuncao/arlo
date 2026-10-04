import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.generation.player_batch_generator import generate_player_batch
from arlo_db_tool.generation.player_draft import PlayerDraft
from arlo_db_tool.generation.team_roster_request_builder import get_team_existing_squad_numbers
from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES
from arlo_db_tool.sql.bulk_player_statement_builder import generate_bulk_player_sql
from arlo_db_tool.ui.bulk.bulk_generation_preview_panel import BulkGenerationPreviewPanel
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.ui.widgets.range_input_widget import RangeInputWidget


class BulkPlayerScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.team_options_map: Dict[str, Optional[str]] = {}
        self.country_options_map: Dict[str, Optional[str]] = {}
        self.last_generated_players: List[PlayerDraft] = []
        self.pos_vars: Dict[str, tk.StringVar] = {}

        top_actions = ttk.Frame(self, padding=(8, 6))
        top_actions.pack(fill=tk.X)

        title_lbl = ttk.Label(
            top_actions,
            text="Geração em Massa de Jogadores",
            font=("TkDefaultFont", 11, "bold"),
        )
        title_lbl.pack(side=tk.LEFT)

        self.btn_reset = ttk.Button(top_actions, text="Resetar", command=self._reset_fields)
        self.btn_reset.pack(side=tk.RIGHT, padx=4)

        self.btn_reload = ttk.Button(top_actions, text="Recarregar Referências", command=self.reload_references)
        self.btn_reload.pack(side=tk.RIGHT, padx=4)

        self.btn_generate = ttk.Button(top_actions, text="Gerar Jogadores & SQL", command=self._generate)
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

        params_frame = ttk.LabelFrame(self.scroll_content, text="Parâmetros de Geração", padding=(10, 8))
        params_frame.pack(fill=tk.X, expand=True, pady=4)
        params_frame.columnconfigure(1, weight=1)
        params_frame.columnconfigure(3, weight=1)

        ttk.Label(params_frame, text="Time de Destino:").grid(row=0, column=0, sticky="w", padx=(4, 6), pady=4)
        self.team_var = tk.StringVar()
        self.combo_team = ttk.Combobox(params_frame, textvariable=self.team_var, state="readonly", width=34)
        self.combo_team.grid(row=0, column=1, sticky="ew", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Faixa de CA (1-200):").grid(row=0, column=2, sticky="w", padx=(4, 6), pady=4)
        self.ca_widget = RangeInputWidget(params_frame, min_limit=1, max_limit=200, default_min=80, default_max=130)
        self.ca_widget.grid(row=0, column=3, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Faixa de Idade:").grid(row=1, column=0, sticky="w", padx=(4, 6), pady=4)
        self.age_widget = RangeInputWidget(params_frame, min_limit=15, max_limit=45, default_min=18, default_max=35)
        self.age_widget.grid(row=1, column=1, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="País Preferencial:").grid(row=1, column=2, sticky="w", padx=(4, 6), pady=4)
        self.country_var = tk.StringVar()
        self.combo_country = ttk.Combobox(params_frame, textvariable=self.country_var, state="readonly", width=34)
        self.combo_country.grid(row=1, column=3, sticky="ew", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Viés País Preferencial (0.0-1.0):").grid(row=2, column=0, sticky="w", padx=(4, 6), pady=4)
        self.country_bias_var = tk.StringVar(value="0.70")
        self.spin_country_bias = ttk.Spinbox(
            params_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.country_bias_var, width=8
        )
        self.spin_country_bias.grid(row=2, column=1, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="Chance Pos. Secundárias:").grid(row=2, column=2, sticky="w", padx=(4, 6), pady=4)
        self.sec_chance_var = tk.StringVar(value="0.40")
        self.spin_sec_chance = ttk.Spinbox(
            params_frame, from_=0.0, to=1.0, increment=0.05, textvariable=self.sec_chance_var, width=8
        )
        self.spin_sec_chance.grid(row=2, column=3, sticky="w", padx=(0, 16), pady=4)

        ttk.Label(params_frame, text="Máx. Pos. Secundárias:").grid(row=3, column=0, sticky="w", padx=(4, 6), pady=4)
        self.max_sec_var = tk.StringVar(value="2")
        self.spin_max_sec = ttk.Spinbox(params_frame, from_=1, to=4, textvariable=self.max_sec_var, width=8)
        self.spin_max_sec.grid(row=3, column=1, sticky="w", padx=(0, 4), pady=4)

        ttk.Label(params_frame, text="Ano de Referência:").grid(row=3, column=2, sticky="w", padx=(4, 6), pady=4)
        self.ref_year_var = tk.StringVar(value="3627")
        self.spin_ref_year = ttk.Spinbox(params_frame, from_=1, to=9999, textvariable=self.ref_year_var, width=8)
        self.spin_ref_year.grid(row=3, column=3, sticky="w", padx=(0, 16), pady=4)

        positions_frame = ttk.LabelFrame(self.scroll_content, text="Quantidade por Posição", padding=(10, 8))
        positions_frame.pack(fill=tk.X, expand=True, pady=4)

        top_pos_bar = ttk.Frame(positions_frame)
        top_pos_bar.pack(fill=tk.X, pady=(0, 8))
        ttk.Label(top_pos_bar, text="Setar valor por posição:").pack(side=tk.LEFT, padx=(0, 6))
        
        self.global_pos_var = tk.StringVar(value="0")
        self.spin_global_pos = ttk.Spinbox(top_pos_bar, from_=0, to=100, textvariable=self.global_pos_var, width=5)
        self.spin_global_pos.pack(side=tk.LEFT, padx=(0, 6))

        def _apply_global_pos():
            val = self.global_pos_var.get().strip()
            if val.isdigit():
                for var in self.pos_vars.values():
                    var.set(val)

        self.btn_apply_global = ttk.Button(top_pos_bar, text="Aplicar a Todas", command=_apply_global_pos)
        self.btn_apply_global.pack(side=tk.LEFT)

        grid_pos_frame = ttk.Frame(positions_frame)
        grid_pos_frame.pack(fill=tk.X)

        for idx, pos_code in enumerate(SHORT_POSITION_CODES):
            r = idx // 7
            c = (idx % 7) * 2
            ttk.Label(grid_pos_frame, text=pos_code).grid(row=r, column=c, sticky="e", padx=(8, 2), pady=4)
            var = tk.StringVar(value="0")
            self.pos_vars[pos_code] = var
            spin = ttk.Spinbox(grid_pos_frame, from_=0, to=100, textvariable=var, width=3)
            spin.grid(row=r, column=c+1, sticky="w", padx=(0, 8), pady=4)

        self.preview_panel = BulkGenerationPreviewPanel(self.scroll_content, title="Prévia do Lote de Jogadores")
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
        team_display = ["(Nenhum / Agente Livre)"]
        self.team_options_map["(Nenhum / Agente Livre)"] = None
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
        self.global_pos_var.set("0")
        for var in self.pos_vars.values():
            var.set("0")
        self.ca_widget.set_range(80, 130)
        self.age_widget.set_range(18, 35)
        self.country_bias_var.set("0.70")
        self.sec_chance_var.set("0.40")
        self.max_sec_var.set("2")
        self.ref_year_var.set("3627")
        self.preview_panel.clear()
        self.sql_output.set_sql("")
        self.last_generated_players = []

    def _generate(self):
        position_quotas = {}
        total_count = 0
        for pos_code, var in self.pos_vars.items():
            try:
                val = int(var.get().strip())
                if val > 0:
                    position_quotas[pos_code] = val
                    total_count += val
            except (ValueError, TypeError):
                pass

        if total_count <= 0:
            messagebox.showerror("Erro", "A quantidade total de jogadores deve ser maior que zero. Defina as quantidades por posição.")
            return

        ca_range = self.ca_widget.get_range()
        age_range = self.age_widget.get_range()

        try:
            bias = float(self.country_bias_var.get().strip())
            sec_chance = float(self.sec_chance_var.get().strip())
            max_sec = int(self.max_sec_var.get().strip())
            ref_year = int(self.ref_year_var.get().strip())
        except (ValueError, TypeError):
            messagebox.showerror("Erro", "Verifique os valores numéricos dos parâmetros.")
            return

        team_id = self.team_options_map.get(self.team_var.get())
        pref_country_id = self.country_options_map.get(self.country_var.get())

        country_opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        available_country_ids = [c_id for c_id, _ in country_opts]

        used_numbers = []
        if team_id and self.app_state.database_path:
            used_numbers = get_team_existing_squad_numbers(self.app_state.database_path, team_id)

        try:
            players = generate_player_batch(
                count=total_count,
                position_quotas=position_quotas,
                team_id=team_id,
                ca_range=(int(ca_range[0]), int(ca_range[1])),
                age_range=(int(age_range[0]), int(age_range[1])),
                reference_year=ref_year,
                available_country_ids=available_country_ids,
                preferred_country_id=pref_country_id,
                preferred_country_bias=bias,
                secondary_positions_chance=sec_chance,
                max_secondary_positions=max_sec,
                used_squad_numbers=used_numbers,
                db_path=self.app_state.database_path,
            )
            self.last_generated_players = players
            sql_text = generate_bulk_player_sql(players)
            self.sql_output.set_sql(sql_text)

            total_positions = sum(len(p.positions) for p in players)
            total_attributes = sum(len(p.attributes) for p in players)

            metrics = [
                ("Total de Jogadores", str(len(players))),
                ("Time Alvo", self.team_var.get()),
                ("Faixa de CA", f"{ca_range[0]} - {ca_range[1]}"),
                ("Faixa de Idade", f"{age_range[0]} - {age_range[1]} anos"),
                ("Total Vínculos Posições", str(total_positions)),
                ("Total Registros Atributos", str(total_attributes)),
            ]
            self.preview_panel.update_preview(
                f"Lote com {len(players)} jogadores gerado com sucesso!",
                metrics,
            )
        except Exception as e:
            messagebox.showerror("Erro ao Gerar Jogadores", str(e))