import tkinter as tk
from tkinter import ttk
from typing import Callable, List, Tuple
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.ui.entry_rule_pool_editor import EntryRulePoolEditor
from arlo_db_tool.ui.schedule_block_editor import ScheduleBlockEditor

STAGE_TYPES = [
    "RoundRobinTable",
    "KnockoutBracket",
    "GroupedCompetitionTable",
]

LEG_FORMATS = [
    "SingleLeg",
    "TwoLegAggregate",
]

class SingleStageEditor(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        stage: StageDraft,
        get_draft_groups_fn: Callable[[], List[Tuple[str, str]]],
        on_delete: Callable[[], None],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, text=f"Fase #{stage.stage_order_index + 1} (Order Index: {stage.stage_order_index})", padding=(8, 6))
        self.app_state = app_state
        self.stage = stage
        self.get_draft_groups = get_draft_groups_fn
        self.on_delete = on_delete
        self.on_change = on_change

        header_frame = ttk.Frame(self)
        header_frame.pack(fill=tk.X, pady=(0, 6))

        ttk.Label(header_frame, text="Tipo de Fase:").pack(side=tk.LEFT, padx=(0, 4))
        self.stage_type_var = tk.StringVar(value=stage.stage_type)
        self.combo_type = ttk.Combobox(
            header_frame,
            textvariable=self.stage_type_var,
            values=STAGE_TYPES,
            state="readonly",
            width=24,
        )
        self.combo_type.pack(side=tk.LEFT, padx=(0, 12))
        self.combo_type.bind("<<ComboboxSelected>>", self._on_type_change)

        self.leg_frame = ttk.Frame(header_frame)
        self.leg_frame.pack(side=tk.LEFT)

        ttk.Label(self.leg_frame, text="Formato de Eliminatória:").pack(side=tk.LEFT, padx=(0, 4))
        self.leg_var = tk.StringVar(value=stage.leg_format or "SingleLeg")
        self.combo_leg = ttk.Combobox(
            self.leg_frame,
            textvariable=self.leg_var,
            values=LEG_FORMATS,
            state="readonly",
            width=18,
        )
        self.combo_leg.pack(side=tk.LEFT)
        self.combo_leg.bind("<<ComboboxSelected>>", self._on_leg_change)

        self.btn_del = ttk.Button(header_frame, text="Excluir Fase", command=self.on_delete)
        self.btn_del.pack(side=tk.RIGHT)

        self.entry_rule_editor = EntryRulePoolEditor(
            self,
            self.app_state,
            self.stage.entry_rule_pools,
            on_change=self.on_change,
        )
        self.entry_rule_editor.pack(fill=tk.BOTH, expand=True, pady=(0, 6))

        self.schedule_block_editor = ScheduleBlockEditor(
            self,
            self.stage.schedule_blocks,
            get_draft_groups_fn=self.get_draft_groups,
            on_change=self.on_change,
        )

        self._update_visibility()

    def _on_type_change(self, _event):
        st = self.stage_type_var.get()
        self.stage.stage_type = st
        if st == "KnockoutBracket":
            if not self.stage.leg_format:
                self.stage.leg_format = "SingleLeg"
                self.leg_var.set("SingleLeg")
        else:
            self.stage.leg_format = None

        self._update_visibility()
        self.on_change()

    def _on_leg_change(self, _event):
        self.stage.leg_format = self.leg_var.get()
        self.on_change()

    def _update_visibility(self):
        st = self.stage.stage_type
        if st == "KnockoutBracket":
            self.leg_frame.pack(side=tk.LEFT)
        else:
            self.leg_frame.pack_forget()

        if st == "GroupedCompetitionTable":
            self.schedule_block_editor.pack(fill=tk.BOTH, expand=True, pady=4)
        else:
            self.schedule_block_editor.pack_forget()

    def reload_references(self):
        self.entry_rule_editor.reload_references()
        self.schedule_block_editor.reload_groups()

    def reload_groups(self):
        self.schedule_block_editor.reload_groups()

class StageListPanel(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        get_draft_groups_fn: Callable[[], List[Tuple[str, str]]],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, text="Fases da Competição (Stages)", padding=(10, 8))
        self.app_state = app_state
        self.get_draft_groups = get_draft_groups_fn
        self.on_change = on_change
        self.stages: List[StageDraft] = []
        self.stage_editors: List[SingleStageEditor] = []

        top_bar = ttk.Frame(self)
        top_bar.pack(fill=tk.X, pady=(0, 6))

        self.btn_add_stage = ttk.Button(top_bar, text="+ Adicionar Fase", command=self._add_stage)
        self.btn_add_stage.pack(side=tk.LEFT)

        self.stages_container = ttk.Frame(self)
        self.stages_container.pack(fill=tk.BOTH, expand=True)

    def _render_stages(self):
        for child in self.stages_container.winfo_children():
            child.destroy()
        self.stage_editors.clear()

        for idx, stage in enumerate(self.stages):
            stage.stage_order_index = idx
            editor = SingleStageEditor(
                self.stages_container,
                self.app_state,
                stage,
                get_draft_groups_fn=self.get_draft_groups,
                on_delete=lambda s=stage: self._delete_stage(s),
                on_change=self.on_change,
            )
            editor.pack(fill=tk.BOTH, expand=True, pady=6)
            self.stage_editors.append(editor)

    def _add_stage(self):
        new_stage = StageDraft(
            id=str(uuid.uuid4()),
            stage_order_index=len(self.stages),
            stage_type="RoundRobinTable",
            leg_format=None,
            entry_rule_pools=[
                EntryRulePoolDraft(
                    id=str(uuid.uuid4()),
                    pool_order_index=0,
                    pool_kind="AllTeams",
                )
            ],
            schedule_blocks=[],
        )
        self.stages.append(new_stage)
        self._render_stages()
        self.on_change()

    def _delete_stage(self, target_stage: StageDraft):
        self.stages.remove(target_stage)
        self._render_stages()
        self.on_change()

    def reload_references(self):
        for editor in self.stage_editors:
            editor.reload_references()

    def reload_groups(self):
        for editor in self.stage_editors:
            editor.reload_groups()

    def populate_from_draft(self, draft: LeagueCalendarConfigDraft):
        self.stages = draft.stages
        self._render_stages()

    def sync_to_draft(self, draft: LeagueCalendarConfigDraft):
        for idx, stage in enumerate(self.stages):
            stage.stage_order_index = idx
        draft.stages = self.stages