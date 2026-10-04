import tkinter as tk
from tkinter import ttk
from typing import Callable, Dict, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_league_calendar_config_options

class LeagueCalendarConfigOperationBar(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        app_state: AppState,
        on_load_config: Optional[Callable[[str], None]] = None,
        on_operation_changed: Optional[Callable[[str], None]] = None,
    ):
        super().__init__(parent, padding=(8, 6))
        self.app_state = app_state
        self.on_load_config = on_load_config or (lambda _id: None)
        self.on_operation_changed = on_operation_changed or (lambda _op: None)

        self.options_map: Dict[str, str] = {}

        lbl_op = ttk.Label(self, text="Modo de Operação:")
        lbl_op.pack(side=tk.LEFT, padx=(0, 6))

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

        self.lbl_target = ttk.Label(self.target_frame, text="Configuração Existente:")
        self.lbl_target.pack(side=tk.LEFT, padx=(0, 6))

        self.target_var = tk.StringVar()
        self.cb_target = ttk.Combobox(
            self.target_frame,
            textvariable=self.target_var,
            state="readonly",
            width=36,
        )
        self.cb_target.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 6))

        self.btn_load = ttk.Button(self.target_frame, text="Carregar", command=self._on_load_clicked)
        self.btn_load.pack(side=tk.LEFT)

        self._update_visibility()
        self.reload_references()

    def _on_op_change(self, _event):
        op = self.op_var.get()
        self._update_visibility()
        self.on_operation_changed(op)

    def _update_visibility(self):
        op = self.op_var.get()
        if op in ("Update", "Delete"):
            self.target_frame.pack(side=tk.LEFT, fill=tk.X, expand=True)
        else:
            self.target_frame.pack_forget()

    def reload_references(self):
        self.options_map.clear()
        options = get_league_calendar_config_options(self.app_state.database_path)

        display_list = []
        for cfg_id, comp_name in options:
            display_text = f"{comp_name} [{cfg_id}]"
            display_list.append(display_text)
            self.options_map[display_text] = cfg_id

        self.cb_target["values"] = display_list
        if display_list:
            if self.target_var.get() not in display_list:
                self.cb_target.current(0)
        else:
            self.target_var.set("")

    def _on_load_clicked(self):
        sel = self.target_var.get()
        cfg_id = self.options_map.get(sel)
        if cfg_id:
            self.on_load_config(cfg_id)

    def get_operation(self) -> str:
        return self.op_var.get()

    def set_operation(self, op: str):
        self.op_var.set(op)
        self._update_visibility()

    def get_selected_config_id(self) -> Optional[str]:
        sel = self.target_var.get()
        return self.options_map.get(sel)