import tkinter as tk
from tkinter import ttk
from typing import Optional, Tuple, Union


class RangeInputWidget(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        label_text: str = "",
        min_limit: Union[int, float] = 0,
        max_limit: Union[int, float] = 200,
        default_min: Union[int, float] = 80,
        default_max: Union[int, float] = 120,
        step: Union[int, float] = 1,
        is_float: bool = False,
    ):
        super().__init__(parent)
        self.min_limit = min_limit
        self.max_limit = max_limit
        self.is_float = is_float

        if label_text:
            self.lbl = ttk.Label(self, text=label_text)
            self.lbl.pack(side=tk.LEFT, padx=(0, 6))

        self.min_var = tk.StringVar(value=str(default_min))
        self.max_var = tk.StringVar(value=str(default_max))

        self.spin_min = ttk.Spinbox(
            self,
            from_=min_limit,
            to=max_limit,
            increment=step,
            textvariable=self.min_var,
            width=6,
        )
        self.spin_min.pack(side=tk.LEFT)

        self.lbl_sep = ttk.Label(self, text="até")
        self.lbl_sep.pack(side=tk.LEFT, padx=4)

        self.spin_max = ttk.Spinbox(
            self,
            from_=min_limit,
            to=max_limit,
            increment=step,
            textvariable=self.max_var,
            width=6,
        )
        self.spin_max.pack(side=tk.LEFT)

    def get_range(self) -> Tuple[Union[int, float], Union[int, float]]:
        try:
            if self.is_float:
                v_min = float(self.min_var.get().strip())
                v_max = float(self.max_var.get().strip())
            else:
                v_min = int(self.min_var.get().strip())
                v_max = int(self.max_var.get().strip())
        except (ValueError, TypeError):
            v_min = self.min_limit
            v_max = self.max_limit

        v_min = max(self.min_limit, min(self.max_limit, v_min))
        v_max = max(self.min_limit, min(self.max_limit, v_max))
        return min(v_min, v_max), max(v_min, v_max)

    def set_range(self, val_min: Union[int, float], val_max: Union[int, float]):
        self.min_var.set(str(val_min))
        self.max_var.set(str(val_max))

    def set_enabled(self, enabled: bool):
        state = "normal" if enabled else "disabled"
        self.spin_min.config(state=state)
        self.spin_max.config(state=state)