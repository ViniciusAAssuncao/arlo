import tkinter as tk
from tkinter import ttk
from typing import Callable, Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec
from arlo_db_tool.schema.registry import get_entity_spec

class OperationSelector(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        on_operation_changed: Callable[[str], None],
        on_target_record_changed: Callable[[Optional[str]], None],
    ):
        super().__init__(parent, padding=(8, 4))
        self.app_state = app_state
        self.on_operation_changed = on_operation_changed
        self.on_target_record_changed = on_target_record_changed

        self.options_map: Dict[str, str] = {}
        self.full_display_list: List[str] = []

        label_op = ttk.Label(self, text="Operação:")
        label_op.pack(side=tk.LEFT, padx=(0, 6))

        self.op_var = tk.StringVar(value="Create")
        self.cb_op = ttk.Combobox(
            self,
            textvariable=self.op_var,
            values=["Create", "Update", "Delete"],
            state="readonly",
            width=12,
        )
        self.cb_op.pack(side=tk.LEFT, padx=(0, 16))
        self.cb_op.bind("<<ComboboxSelected>>", self._on_op_change)

        self.target_frame = ttk.Frame(self)
        self.target_frame.pack(side=tk.LEFT, fill=tk.X, expand=True)

        self.lbl_search = ttk.Label(self.target_frame, text="Buscar:")
        self.lbl_search.pack(side=tk.LEFT, padx=(0, 6))

        self.search_var = tk.StringVar()
        self.search_var.trace_add("write", self._on_search)
        self.entry_search = ttk.Entry(
            self.target_frame, 
            textvariable=self.search_var, 
            width=20
        )
        self.entry_search.pack(side=tk.LEFT, padx=(0, 16))

        self.lbl_target = ttk.Label(self.target_frame, text="Registro Alvo:")
        self.lbl_target.pack(side=tk.LEFT, padx=(0, 6))

        self.target_var = tk.StringVar()
        self.cb_target = ttk.Combobox(
            self.target_frame,
            textvariable=self.target_var,
            state="readonly",
            width=36,
        )
        self.cb_target.pack(side=tk.LEFT, fill=tk.X, expand=True)
        self.cb_target.bind("<<ComboboxSelected>>", self._on_target_select)

        self._update_visibility()

    def _on_op_change(self, _event):
        op = self.op_var.get()
        self.app_state.current_operation = op
        self._update_visibility()
        self.reload_target_records()
        self.on_operation_changed(op)

    def _on_search(self, *args):
        self._apply_filter()

    def _apply_filter(self):
        search_term = self.search_var.get().strip().lower()
        if not search_term:
            filtered = self.full_display_list
        else:
            filtered = [item for item in self.full_display_list if search_term in item.lower()]

        self.cb_target["values"] = filtered
        if filtered:
            if self.target_var.get() not in filtered:
                self.cb_target.current(0)
            target_id = self.options_map[self.target_var.get()]
            self.app_state.selected_record_id = target_id
            self.on_target_record_changed(target_id)
        else:
            self.target_var.set("")
            self.app_state.selected_record_id = None
            self.on_target_record_changed(None)

    def _on_target_select(self, _event):
        sel = self.target_var.get()
        target_id = self.options_map.get(sel)
        self.app_state.selected_record_id = target_id
        self.on_target_record_changed(target_id)

    def _update_visibility(self):
        op = self.op_var.get()
        if op in ("Update", "Delete"):
            self.target_frame.pack(side=tk.LEFT, fill=tk.X, expand=True)
        else:
            self.target_frame.pack_forget()

    def reload_target_records(self):
        self.options_map.clear()
        self.full_display_list.clear()
        
        entity_name = self.app_state.current_entity
        if not entity_name or not self.app_state.database_path:
            self.cb_target["values"] = []
            self.target_var.set("")
            return

        spec = get_entity_spec(entity_name)
        if not spec:
            self.cb_target["values"] = []
            self.target_var.set("")
            return

        where_filter = None
        if isinstance(spec, CompositeEntitySpec):
            main_spec = spec.specs[0]
            table_name = main_spec.table_name
            pk_field = main_spec.primary_key_field()
            if spec.name == "Manager":
                where_filter = "id IN (SELECT id FROM managers)"
            elif spec.name == "Referee":
                where_filter = "id IN (SELECT id FROM referees)"
        else:
            table_name = spec.table_name
            pk_field = spec.primary_key_field()

        pk_col = pk_field.column if pk_field else "id"

        raw_options = get_options(
            self.app_state.database_path,
            table_name,
            id_col=pk_col,
            where=where_filter,
        )
        
        for row_id, label in raw_options:
            display_text = f"{label} [{row_id}]"
            self.full_display_list.append(display_text)
            self.options_map[display_text] = str(row_id)

        self.search_var.set("")
        self._apply_filter()

    def get_selected_operation(self) -> str:
        return self.op_var.get()

    def get_selected_target_id(self) -> Optional[str]:
        return self.app_state.selected_record_id