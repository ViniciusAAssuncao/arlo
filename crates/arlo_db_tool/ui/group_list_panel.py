import tkinter as tk
from tkinter import ttk
from typing import Callable, Dict, List, Optional, Tuple
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.domain.competition_group_draft import CompetitionGroupDraft
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

class SingleGroupEditor(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        group: CompetitionGroupDraft,
        available_teams: List[Tuple[str, str]],
        on_delete: Callable[[], None],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, text=f"Grupo #{group.order_index + 1}", padding=(8, 6))
        self.group = group
        self.available_teams = available_teams
        self.on_delete = on_delete
        self.on_change = on_change

        top_row = ttk.Frame(self)
        top_row.pack(fill=tk.X, pady=(0, 6))

        lbl_name = ttk.Label(top_row, text="Nome do Grupo:")
        lbl_name.pack(side=tk.LEFT, padx=(0, 6))

        self.name_var = tk.StringVar(value=group.name)
        self.name_var.trace_add("write", lambda *_: self._sync_name())
        self.entry_name = ttk.Entry(top_row, textvariable=self.name_var, width=24)
        self.entry_name.pack(side=tk.LEFT, padx=(0, 12))

        self.btn_del = ttk.Button(top_row, text="Excluir Grupo", command=self.on_delete)
        self.btn_del.pack(side=tk.RIGHT)

        selector_frame = ttk.Frame(self)
        selector_frame.pack(fill=tk.BOTH, expand=True)

        avail_col = ttk.Frame(selector_frame)
        avail_col.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 4))

        ttk.Label(avail_col, text="Times Disponíveis no Banco:").pack(anchor="w")
        self.lb_avail = tk.Listbox(avail_col, height=5, selectmode=tk.EXTENDED)
        self.lb_avail.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        sb_avail = ttk.Scrollbar(avail_col, orient=tk.VERTICAL, command=self.lb_avail.yview)
        sb_avail.pack(side=tk.RIGHT, fill=tk.Y)
        self.lb_avail.config(yscrollcommand=sb_avail.set)

        mid_col = ttk.Frame(selector_frame)
        mid_col.pack(side=tk.LEFT, padx=6, pady=10)

        self.btn_add_team = ttk.Button(mid_col, text=">>", width=4, command=self._add_selected_teams)
        self.btn_add_team.pack(pady=4)

        self.btn_rem_team = ttk.Button(mid_col, text="<<", width=4, command=self._remove_selected_teams)
        self.btn_rem_team.pack(pady=4)

        sel_col = ttk.Frame(selector_frame)
        sel_col.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(4, 0))

        ttk.Label(sel_col, text="Times do Grupo:").pack(anchor="w")
        self.lb_selected = tk.Listbox(sel_col, height=5, selectmode=tk.EXTENDED)
        self.lb_selected.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        sb_selected = ttk.Scrollbar(sel_col, orient=tk.VERTICAL, command=self.lb_selected.yview)
        sb_selected.pack(side=tk.RIGHT, fill=tk.Y)
        self.lb_selected.config(yscrollcommand=sb_selected.set)

        self._refresh_team_lists()

    def _sync_name(self):
        self.group.name = self.name_var.get().strip()
        self.on_change()

    def update_available_teams(self, teams: List[Tuple[str, str]]):
        self.available_teams = teams
        self._refresh_team_lists()

    def _refresh_team_lists(self):
        selected_set = set(self.group.team_ids)
        team_map = {t_id: label for t_id, label in self.available_teams}

        self.lb_avail.delete(0, tk.END)
        self.avail_ids = []
        for t_id, label in self.available_teams:
            if t_id not in selected_set:
                self.lb_avail.insert(tk.END, f"{label} [{t_id}]")
                self.avail_ids.append(t_id)

        self.lb_selected.delete(0, tk.END)
        self.selected_ids = []
        for t_id in self.group.team_ids:
            label = team_map.get(t_id, t_id)
            self.lb_selected.insert(tk.END, f"{label} [{t_id}]")
            self.selected_ids.append(t_id)

    def _add_selected_teams(self):
        selections = self.lb_avail.curselection()
        for idx in selections:
            t_id = self.avail_ids[idx]
            if t_id not in self.group.team_ids:
                self.group.team_ids.append(t_id)
        self._refresh_team_lists()
        self.on_change()

    def _remove_selected_teams(self):
        selections = self.lb_selected.curselection()
        to_remove = {self.selected_ids[idx] for idx in selections}
        self.group.team_ids = [tid for tid in self.group.team_ids if tid not in to_remove]
        self._refresh_team_lists()
        self.on_change()

class GroupListPanel(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        on_groups_changed: Optional[Callable[[], None]] = None,
    ):
        super().__init__(parent, text="Grupos da Competição (Opcional)", padding=(10, 8))
        self.app_state = app_state
        self.on_groups_changed = on_groups_changed or (lambda: None)
        self.groups: List[CompetitionGroupDraft] = []
        self.available_teams: List[Tuple[str, str]] = []
        self.editor_widgets: List[SingleGroupEditor] = []

        top_bar = ttk.Frame(self)
        top_bar.pack(fill=tk.X, pady=(0, 6))

        self.btn_add_group = ttk.Button(top_bar, text="+ Adicionar Grupo", command=self._add_group)
        self.btn_add_group.pack(side=tk.LEFT)

        self.items_container = ttk.Frame(self)
        self.items_container.pack(fill=tk.BOTH, expand=True)

        self.reload_references()

    def reload_references(self):
        self.available_teams = get_options(
            self.app_state.database_path,
            "teams",
            id_col="id",
            label_expr="name",
        )
        for editor in self.editor_widgets:
            editor.update_available_teams(self.available_teams)

    def _render_editors(self):
        for child in self.items_container.winfo_children():
            child.destroy()
        self.editor_widgets.clear()

        for idx, group in enumerate(self.groups):
            group.order_index = idx
            editor = SingleGroupEditor(
                self.items_container,
                group,
                self.available_teams,
                on_delete=lambda g=group: self._delete_group(g),
                on_change=self.on_groups_changed,
            )
            editor.pack(fill=tk.X, expand=True, pady=4)
            self.editor_widgets.append(editor)

    def _add_group(self):
        new_group = CompetitionGroupDraft(
            id=str(uuid.uuid4()),
            order_index=len(self.groups),
            name=f"Grupo {chr(65 + len(self.groups)) if len(self.groups) < 26 else len(self.groups) + 1}",
            team_ids=[],
        )
        self.groups.append(new_group)
        self._render_editors()
        self.on_groups_changed()

    def _delete_group(self, target_group: CompetitionGroupDraft):
        self.groups = [g for g in self.groups if g.id != target_group.id]
        self._render_editors()
        self.on_groups_changed()

    def populate_from_draft(self, draft: LeagueCalendarConfigDraft):
        self.groups = draft.groups
        self._render_editors()

    def sync_to_draft(self, draft: LeagueCalendarConfigDraft):
        for idx, group in enumerate(self.groups):
            group.order_index = idx
        draft.groups = self.groups