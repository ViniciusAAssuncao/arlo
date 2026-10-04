import tkinter as tk
from tkinter import ttk
from typing import Callable, List, Tuple
import uuid
from arlo_db_tool.domain.schedule_block_draft import ScheduleBlockDraft

BLOCK_KINDS = [
    "GroupRoundRobin",
    "CrossGroupPairing",
    "RandomPoolRounds",
]

class SingleScheduleBlockRow(ttk.Frame):
    def __init__(
        self,
        parent: tk.Widget,
        block: ScheduleBlockDraft,
        get_draft_groups_fn: Callable[[], List[Tuple[str, str]]],
        on_delete: Callable[[], None],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, padding=(4, 4))
        self.block = block
        self.get_draft_groups = get_draft_groups_fn
        self.on_delete = on_delete
        self.on_change = on_change

        self.kind_var = tk.StringVar(value=block.block_kind)
        self.combo_kind = ttk.Combobox(
            self,
            textvariable=self.kind_var,
            values=BLOCK_KINDS,
            state="readonly",
            width=20,
        )
        self.combo_kind.pack(side=tk.LEFT, padx=(0, 6))
        self.combo_kind.bind("<<ComboboxSelected>>", self._on_kind_change)

        self.dyn_container = ttk.Frame(self)
        self.dyn_container.pack(side=tk.LEFT, fill=tk.X, expand=True)

        self.btn_del = ttk.Button(self, text="✕", width=3, command=self.on_delete)
        self.btn_del.pack(side=tk.RIGHT, padx=(6, 0))

        self._build_dynamic_fields()

    def _on_kind_change(self, _event):
        self.block.block_kind = self.kind_var.get()
        self._build_dynamic_fields()
        self.on_change()

    def reload_groups(self):
        self._build_dynamic_fields()

    def _build_dynamic_fields(self):
        for child in self.dyn_container.winfo_children():
            child.destroy()

        kind = self.block.block_kind
        groups = self.get_draft_groups()
        group_map = {label: gid for gid, label in groups}
        reverse_map = {gid: label for gid, label in groups}

        if kind == "GroupRoundRobin":
            ttk.Label(self.dyn_container, text="Grupo:").pack(side=tk.LEFT, padx=(0, 4))
            curr_sel = reverse_map.get(self.block.group_a_id or "", "")
            grp_var = tk.StringVar(value=curr_sel)

            def _sync_g(_event):
                self.block.group_a_id = group_map.get(grp_var.get(), None)
                self.on_change()

            cb = ttk.Combobox(self.dyn_container, textvariable=grp_var, values=list(group_map.keys()), state="readonly", width=18)
            cb.pack(side=tk.LEFT)
            cb.bind("<<ComboboxSelected>>", _sync_g)

        elif kind == "CrossGroupPairing":
            ttk.Label(self.dyn_container, text="Grupo A:").pack(side=tk.LEFT, padx=(0, 4))
            curr_a = reverse_map.get(self.block.group_a_id or "", "")
            var_a = tk.StringVar(value=curr_a)

            def _sync_a(_event):
                self.block.group_a_id = group_map.get(var_a.get(), None)
                self.on_change()

            cb_a = ttk.Combobox(self.dyn_container, textvariable=var_a, values=list(group_map.keys()), state="readonly", width=14)
            cb_a.pack(side=tk.LEFT, padx=(0, 6))
            cb_a.bind("<<ComboboxSelected>>", _sync_a)

            ttk.Label(self.dyn_container, text="Grupo B:").pack(side=tk.LEFT, padx=(0, 4))
            curr_b = reverse_map.get(self.block.group_b_id or "", "")
            var_b = tk.StringVar(value=curr_b)

            def _sync_b(_event):
                self.block.group_b_id = group_map.get(var_b.get(), None)
                self.on_change()

            cb_b = ttk.Combobox(self.dyn_container, textvariable=var_b, values=list(group_map.keys()), state="readonly", width=14)
            cb_b.pack(side=tk.LEFT, padx=(0, 6))
            cb_b.bind("<<ComboboxSelected>>", _sync_b)

            mirr_var = tk.BooleanVar(value=bool(self.block.mirrored))

            def _sync_mirr():
                self.block.mirrored = mirr_var.get()
                self.on_change()

            chk_mirr = ttk.Checkbutton(self.dyn_container, text="Espelhado (Ida & Volta)", variable=mirr_var, command=_sync_mirr)
            chk_mirr.pack(side=tk.LEFT)

        elif kind == "RandomPoolRounds":
            ttk.Label(self.dyn_container, text="Pool:").pack(side=tk.LEFT, padx=(0, 4))
            pool_kind_var = tk.StringVar(value=self.block.pool_kind or "AllGroups")

            cb_pool_kind = ttk.Combobox(
                self.dyn_container,
                textvariable=pool_kind_var,
                values=["AllGroups", "SpecificGroups"],
                state="readonly",
                width=14,
            )
            cb_pool_kind.pack(side=tk.LEFT, padx=(0, 6))

            ttk.Label(self.dyn_container, text="Rodadas:").pack(side=tk.LEFT, padx=(0, 4))
            rounds_var = tk.StringVar(value=str(self.block.rounds_count if self.block.rounds_count is not None else 1))

            def _sync_rounds(*_):
                try:
                    self.block.rounds_count = int(rounds_var.get())
                except ValueError:
                    self.block.rounds_count = None
                self.on_change()

            rounds_var.trace_add("write", _sync_rounds)
            spin_r = ttk.Spinbox(self.dyn_container, from_=1, to=50, textvariable=rounds_var, width=5)
            spin_r.pack(side=tk.LEFT, padx=(0, 6))

            spec_frame = ttk.Frame(self.dyn_container)
            spec_frame.pack(side=tk.LEFT)

            def _update_pool_kind_view(_event=None):
                pk = pool_kind_var.get()
                self.block.pool_kind = pk
                for child in spec_frame.winfo_children():
                    child.destroy()
                if pk == "SpecificGroups":
                    ttk.Label(spec_frame, text="Grupos:").pack(side=tk.LEFT, padx=(4, 2))
                    for gid, glbl in groups:
                        is_sel = gid in self.block.pool_group_ids
                        g_chk_var = tk.BooleanVar(value=is_sel)

                        def _make_sync_g(target_gid, var):
                            def _fn():
                                if var.get():
                                    if target_gid not in self.block.pool_group_ids:
                                        self.block.pool_group_ids.append(target_gid)
                                else:
                                    if target_gid in self.block.pool_group_ids:
                                        self.block.pool_group_ids.remove(target_gid)
                                self.on_change()
                            return _fn

                        c = ttk.Checkbutton(spec_frame, text=glbl, variable=g_chk_var, command=_make_sync_g(gid, g_chk_var))
                        c.pack(side=tk.LEFT, padx=2)
                self.on_change()

            cb_pool_kind.bind("<<ComboboxSelected>>", _update_pool_kind_view)
            _update_pool_kind_view()

class ScheduleBlockEditor(ttk.LabelFrame):
    def __init__(
        self,
        parent: tk.Widget,
        blocks: List[ScheduleBlockDraft],
        get_draft_groups_fn: Callable[[], List[Tuple[str, str]]],
        on_change: Callable[[], None],
    ):
        super().__init__(parent, text="Blocos de Agendamento (Schedule Blocks)", padding=(8, 6))
        self.blocks = blocks
        self.get_draft_groups = get_draft_groups_fn
        self.on_change = on_change
        self.row_widgets: List[SingleScheduleBlockRow] = []

        top_bar = ttk.Frame(self)
        top_bar.pack(fill=tk.X, pady=(0, 4))

        self.btn_add = ttk.Button(top_bar, text="+ Adicionar Bloco", command=self._add_block)
        self.btn_add.pack(side=tk.LEFT)

        self.rows_container = ttk.Frame(self)
        self.rows_container.pack(fill=tk.BOTH, expand=True)

        self._render_rows()

    def reload_groups(self):
        for row in self.row_widgets:
            row.reload_groups()

    def _render_rows(self):
        for child in self.rows_container.winfo_children():
            child.destroy()
        self.row_widgets.clear()

        for idx, block in enumerate(self.blocks):
            block.block_order_index = idx
            row_w = SingleScheduleBlockRow(
                self.rows_container,
                block,
                self.get_draft_groups,
                on_delete=lambda b=block: self._delete_block(b),
                on_change=self.on_change,
            )
            row_w.pack(fill=tk.X, expand=True, pady=2)
            self.row_widgets.append(row_w)

    def _add_block(self):
        new_block = ScheduleBlockDraft(
            id=str(uuid.uuid4()),
            block_order_index=len(self.blocks),
            block_kind="GroupRoundRobin",
        )
        self.blocks.append(new_block)
        self._render_rows()
        self.on_change()

    def _delete_block(self, target_block: ScheduleBlockDraft):
        self.blocks.remove(target_block)
        self._render_rows()
        self.on_change()

    def set_blocks(self, blocks: List[ScheduleBlockDraft]):
        self.blocks = blocks
        self._render_rows()