import tkinter as tk
from tkinter import ttk
from typing import Any, Dict
from arlo_db_tool.app_state import AppState
from arlo_db_tool.domain.config_core_fields import CONFIG_CORE_FIELDS
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.ui.widgets.widget_factory import FieldWidget, create_field_widget

class ConfigCorePanel(ttk.LabelFrame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent, text="Configuração Principal da Liga", padding=(10, 8))
        self.app_state = app_state
        self.field_widgets: Dict[str, FieldWidget] = {}

        self._build_sections()

    def _build_sections(self):
        fields_by_name = {f.name: f for f in CONFIG_CORE_FIELDS}

        general_frame = ttk.LabelFrame(self, text="Geral & Formato da Temporada", padding=(8, 6))
        general_frame.pack(fill=tk.X, padx=4, pady=4)
        general_frame.columnconfigure(1, weight=1)
        general_frame.columnconfigure(3, weight=1)

        general_names = [
            ("id", "competition_id"),
            ("algorithm", "season_length_weeks"),
            ("season_start_month_order_index", "season_start_day_of_month"),
            ("max_games_per_team_per_week", "games_per_week_conflict_scope"),
            ("postponement_strategy_kind", "standings_stage_order_index"),
            ("neutral_opener_enabled", "neutral_opener_selection_strategy"),
        ]

        for r_idx, (col0_name, col1_name) in enumerate(general_names):
            f0 = fields_by_name[col0_name]
            lbl0 = ttk.Label(general_frame, text=f"{f0.name}:")
            lbl0.grid(row=r_idx, column=0, sticky="w", padx=(4, 6), pady=4)
            w0 = create_field_widget(general_frame, f0, self.app_state)
            w0.container.grid(row=r_idx, column=1, sticky="ew", padx=(0, 16), pady=4)
            self.field_widgets[f0.name] = w0

            f1 = fields_by_name[col1_name]
            lbl1 = ttk.Label(general_frame, text=f"{f1.name}:")
            lbl1.grid(row=r_idx, column=2, sticky="w", padx=(4, 6), pady=4)
            w1 = create_field_widget(general_frame, f1, self.app_state)
            w1.container.grid(row=r_idx, column=3, sticky="ew", padx=(0, 4), pady=4)
            self.field_widgets[f1.name] = w1

        metrics_frame = ttk.LabelFrame(self, text="Pesos de Classificação (SPA & QTA)", padding=(8, 6))
        metrics_frame.pack(fill=tk.X, padx=4, pady=4)
        metrics_frame.columnconfigure(1, weight=1)
        metrics_frame.columnconfigure(3, weight=1)

        metrics_names = [
            ("spa_win_weight", "spa_draw_weight"),
            ("spa_loss_weight", "spa_feo_k_factor"),
            ("qta_home_win_weight", "qta_away_win_weight"),
            ("qta_home_draw_weight", "qta_away_draw_weight"),
            ("qta_home_loss_weight", "qta_away_loss_weight"),
        ]

        for r_idx, (col0_name, col1_name) in enumerate(metrics_names):
            f0 = fields_by_name[col0_name]
            lbl0 = ttk.Label(metrics_frame, text=f"{f0.name}:")
            lbl0.grid(row=r_idx, column=0, sticky="w", padx=(4, 6), pady=4)
            w0 = create_field_widget(metrics_frame, f0, self.app_state)
            w0.container.grid(row=r_idx, column=1, sticky="ew", padx=(0, 16), pady=4)
            self.field_widgets[f0.name] = w0

            f1 = fields_by_name[col1_name]
            lbl1 = ttk.Label(metrics_frame, text=f"{f1.name}:")
            lbl1.grid(row=r_idx, column=2, sticky="w", padx=(4, 6), pady=4)
            w1 = create_field_widget(metrics_frame, f1, self.app_state)
            w1.container.grid(row=r_idx, column=3, sticky="ew", padx=(0, 4), pady=4)
            self.field_widgets[f1.name] = w1

        prom_frame = ttk.LabelFrame(self, text="Promoção & Rebaixamento", padding=(8, 6))
        prom_frame.pack(fill=tk.X, padx=4, pady=4)
        prom_frame.columnconfigure(1, weight=1)
        prom_frame.columnconfigure(3, weight=1)

        prom_names = [
            ("promotion_rule_kind", "relegation_rule_kind"),
            ("promotion_count", "relegation_count"),
            ("promotion_playoff_stage_order_index", "relegation_playoff_stage_order_index"),
            ("promotion_target_league_id", "relegation_target_league_id"),
        ]

        for r_idx, (col0_name, col1_name) in enumerate(prom_names):
            f0 = fields_by_name[col0_name]
            lbl0 = ttk.Label(prom_frame, text=f"{f0.name}:")
            lbl0.grid(row=r_idx, column=0, sticky="w", padx=(4, 6), pady=4)
            w0 = create_field_widget(prom_frame, f0, self.app_state)
            w0.container.grid(row=r_idx, column=1, sticky="ew", padx=(0, 16), pady=4)
            self.field_widgets[f0.name] = w0

            f1 = fields_by_name[col1_name]
            lbl1 = ttk.Label(prom_frame, text=f"{f1.name}:")
            lbl1.grid(row=r_idx, column=2, sticky="w", padx=(4, 6), pady=4)
            w1 = create_field_widget(prom_frame, f1, self.app_state)
            w1.container.grid(row=r_idx, column=3, sticky="ew", padx=(0, 4), pady=4)
            self.field_widgets[f1.name] = w1

    def populate_from_draft(self, draft: LeagueCalendarConfigDraft):
        for field in CONFIG_CORE_FIELDS:
            fw = self.field_widgets.get(field.name)
            if not fw:
                continue
            val = getattr(draft, field.name, None)
            fw.set_value(val)

    def sync_to_draft(self, draft: LeagueCalendarConfigDraft):
        for field in CONFIG_CORE_FIELDS:
            fw = self.field_widgets.get(field.name)
            if not fw:
                continue
            val = fw.get_value()
            if hasattr(draft, field.name):
                setattr(draft, field.name, val)

    def reload_references(self):
        for fw in self.field_widgets.values():
            fw.reload()