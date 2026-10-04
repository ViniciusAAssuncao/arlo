import tkinter as tk
from tkinter import ttk
from typing import List
import uuid
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft

ALL_CRITERIA = [
    "IspaTotal",
    "QtaScore",
    "GoalDifference",
    "GoalPointsTotal",
    "HeadToHead",
    "Random",
]

class TieBreakCriteriaPanel(ttk.LabelFrame):
    def __init__(self, parent: tk.Widget):
        super().__init__(parent, text="Critérios de Desempate (Ordem de Prioridade)", padding=(10, 8))

        self.criteria_list: List[str] = list(ALL_CRITERIA)

        content = ttk.Frame(self)
        content.pack(fill=tk.BOTH, expand=True)

        left = ttk.Frame(content)
        left.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 8))

        self.listbox = tk.Listbox(left, height=6, selectmode=tk.SINGLE)
        self.listbox.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        scrollbar = ttk.Scrollbar(left, orient=tk.VERTICAL, command=self.listbox.yview)
        scrollbar.pack(side=tk.RIGHT, fill=tk.Y)
        self.listbox.config(yscrollcommand=scrollbar.set)

        right = ttk.Frame(content)
        right.pack(side=tk.RIGHT, fill=tk.Y)

        self.btn_up = ttk.Button(right, text="▲ Subir", command=self._move_up, width=10)
        self.btn_up.pack(pady=2)

        self.btn_down = ttk.Button(right, text="▼ Descer", command=self._move_down, width=10)
        self.btn_down.pack(pady=2)

        self.btn_remove = ttk.Button(right, text="Remover", command=self._remove_selected, width=10)
        self.btn_remove.pack(pady=2)

        add_frame = ttk.Frame(self)
        add_frame.pack(fill=tk.X, pady=(8, 0))

        lbl = ttk.Label(add_frame, text="Adicionar Critério:")
        lbl.pack(side=tk.LEFT, padx=(0, 6))

        self.add_var = tk.StringVar(value=ALL_CRITERIA[0])
        self.combo_add = ttk.Combobox(
            add_frame,
            textvariable=self.add_var,
            values=ALL_CRITERIA,
            state="readonly",
            width=22,
        )
        self.combo_add.pack(side=tk.LEFT, padx=(0, 6))

        self.btn_add = ttk.Button(add_frame, text="Adicionar", command=self._add_criterion)
        self.btn_add.pack(side=tk.LEFT)

        self._refresh_listbox()

    def _refresh_listbox(self):
        self.listbox.delete(0, tk.END)
        for idx, item in enumerate(self.criteria_list):
            self.listbox.insert(tk.END, f"{idx + 1}. {item}")

    def _move_up(self):
        sel = self.listbox.curselection()
        if not sel or sel[0] == 0:
            return
        idx = sel[0]
        self.criteria_list[idx - 1], self.criteria_list[idx] = (
            self.criteria_list[idx],
            self.criteria_list[idx - 1],
        )
        self._refresh_listbox()
        self.listbox.selection_set(idx - 1)

    def _move_down(self):
        sel = self.listbox.curselection()
        if not sel or sel[0] >= len(self.criteria_list) - 1:
            return
        idx = sel[0]
        self.criteria_list[idx + 1], self.criteria_list[idx] = (
            self.criteria_list[idx],
            self.criteria_list[idx + 1],
        )
        self._refresh_listbox()
        self.listbox.selection_set(idx + 1)

    def _remove_selected(self):
        sel = self.listbox.curselection()
        if not sel:
            return
        idx = sel[0]
        self.criteria_list.pop(idx)
        self._refresh_listbox()
        if self.criteria_list:
            next_sel = min(idx, len(self.criteria_list) - 1)
            self.listbox.selection_set(next_sel)

    def _add_criterion(self):
        val = self.add_var.get()
        if val:
            self.criteria_list.append(val)
            self._refresh_listbox()
            self.listbox.selection_set(len(self.criteria_list) - 1)

    def populate_from_draft(self, draft: LeagueCalendarConfigDraft):
        sorted_criteria = sorted(draft.tie_break_criteria, key=lambda c: c.order_index)
        self.criteria_list = [c.criterion_kind for c in sorted_criteria]
        self._refresh_listbox()

    def sync_to_draft(self, draft: LeagueCalendarConfigDraft):
        criteria = []
        for idx, kind in enumerate(self.criteria_list):
            criteria.append(
                TieBreakCriterionDraft(
                    id=str(uuid.uuid4()),
                    order_index=idx,
                    criterion_kind=kind,
                )
            )
        draft.tie_break_criteria = criteria