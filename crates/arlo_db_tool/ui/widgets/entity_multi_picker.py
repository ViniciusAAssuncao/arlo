import tkinter as tk
from tkinter import ttk
from typing import Callable, List, Optional, Tuple


class EntityMultiPicker(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        left_title: str = "Disponíveis:",
        right_title: str = "Selecionados:",
        height: int = 6,
        on_change: Optional[Callable[[], None]] = None,
    ):
        super().__init__(parent)
        self.on_change = on_change or (lambda: None)
        self.all_items: List[Tuple[str, str]] = []
        self.selected_ids_list: List[str] = []
        self.avail_ids: List[str] = []
        self.selected_ids: List[str] = []

        avail_col = ttk.Frame(self)
        avail_col.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 4))

        ttk.Label(avail_col, text=left_title).pack(anchor="w")
        self.lb_avail = tk.Listbox(avail_col, height=height, selectmode=tk.EXTENDED)
        self.lb_avail.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        sb_avail = ttk.Scrollbar(avail_col, orient=tk.VERTICAL, command=self.lb_avail.yview)
        sb_avail.pack(side=tk.RIGHT, fill=tk.Y)
        self.lb_avail.config(yscrollcommand=sb_avail.set)

        mid_col = ttk.Frame(self)
        mid_col.pack(side=tk.LEFT, padx=6, pady=6)

        self.btn_add = ttk.Button(mid_col, text=">>", width=4, command=self._add_selected)
        self.btn_add.pack(pady=3)

        self.btn_rem = ttk.Button(mid_col, text="<<", width=4, command=self._remove_selected)
        self.btn_rem.pack(pady=3)

        sel_col = ttk.Frame(self)
        sel_col.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(4, 0))

        ttk.Label(sel_col, text=right_title).pack(anchor="w")
        self.lb_selected = tk.Listbox(sel_col, height=height, selectmode=tk.EXTENDED)
        self.lb_selected.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        sb_selected = ttk.Scrollbar(sel_col, orient=tk.VERTICAL, command=self.lb_selected.yview)
        sb_selected.pack(side=tk.RIGHT, fill=tk.Y)
        self.lb_selected.config(yscrollcommand=sb_selected.set)

    def set_available_items(self, items: List[Tuple[str, str]]):
        self.all_items = list(items)
        self._refresh_lists()

    def get_selected_ids(self) -> List[str]:
        return list(self.selected_ids_list)

    def set_selected_ids(self, ids: List[str]):
        self.selected_ids_list = list(ids)
        self._refresh_lists()

    def clear(self):
        self.selected_ids_list.clear()
        self._refresh_lists()

    def _refresh_lists(self):
        selected_set = set(self.selected_ids_list)
        label_map = {item_id: lbl for item_id, lbl in self.all_items}

        self.lb_avail.delete(0, tk.END)
        self.avail_ids = []
        for item_id, label in self.all_items:
            if item_id not in selected_set:
                self.lb_avail.insert(tk.END, f"{label} [{item_id}]")
                self.avail_ids.append(item_id)

        self.lb_selected.delete(0, tk.END)
        self.selected_ids = []
        for item_id in self.selected_ids_list:
            label = label_map.get(item_id, item_id)
            self.lb_selected.insert(tk.END, f"{label} [{item_id}]")
            self.selected_ids.append(item_id)

    def _add_selected(self):
        selections = self.lb_avail.curselection()
        for idx in selections:
            item_id = self.avail_ids[idx]
            if item_id not in self.selected_ids_list:
                self.selected_ids_list.append(item_id)
        self._refresh_lists()
        self.on_change()

    def _remove_selected(self):
        selections = self.lb_selected.curselection()
        to_remove = {self.selected_ids[idx] for idx in selections}
        self.selected_ids_list = [i for i in self.selected_ids_list if i not in to_remove]
        self._refresh_lists()
        self.on_change()

    def set_enabled(self, enabled: bool):
        state = "normal" if enabled else "disabled"
        self.btn_add.config(state=state)
        self.btn_rem.config(state=state)
        self.lb_avail.config(state=state)
        self.lb_selected.config(state=state)