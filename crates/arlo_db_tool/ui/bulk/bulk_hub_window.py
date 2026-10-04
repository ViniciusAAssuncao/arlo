import tkinter as tk
from tkinter import ttk
from typing import Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.ui.bulk.bulk_deletion_screen import BulkDeletionScreen
from arlo_db_tool.ui.bulk.bulk_manager_screen import BulkManagerScreen
from arlo_db_tool.ui.bulk.bulk_player_screen import BulkPlayerScreen
from arlo_db_tool.ui.bulk.bulk_referee_screen import BulkRefereeScreen
from arlo_db_tool.ui.bulk.bulk_team_roster_screen import BulkTeamRosterScreen
from arlo_db_tool.ui.bulk.bulk_venue_screen import BulkVenueScreen
from arlo_db_tool.ui.bulk.league_population_wizard_screen import (
    LeaguePopulationWizardScreen,
)


class BulkHubWindow(tk.Toplevel):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.title("Ferramentas em Massa (Bulk Tools)")
        self.geometry("1120x840")
        self.minsize(850, 600)

        self.app_state = app_state

        self.notebook = ttk.Notebook(self)
        self.notebook.pack(fill=tk.BOTH, expand=True, padx=4, pady=4)

        self.wizard_screen = LeaguePopulationWizardScreen(self.notebook, self.app_state)
        self.notebook.add(self.wizard_screen, text="Popular Liga Completa")

        self.venue_screen = BulkVenueScreen(self.notebook, self.app_state)
        self.notebook.add(self.venue_screen, text="Estádios / Venues (Lote)")

        self.player_screen = BulkPlayerScreen(self.notebook, self.app_state)
        self.notebook.add(self.player_screen, text="Jogadores (Lote)")

        self.roster_screen = BulkTeamRosterScreen(self.notebook, self.app_state)
        self.notebook.add(self.roster_screen, text="Elenco de Time Completo")

        self.manager_screen = BulkManagerScreen(self.notebook, self.app_state)
        self.notebook.add(self.manager_screen, text="Técnicos (Lote)")

        self.referee_screen = BulkRefereeScreen(self.notebook, self.app_state)
        self.notebook.add(self.referee_screen, text="Juízes (Lote)")

        self.deletion_screen = BulkDeletionScreen(self.notebook, self.app_state)
        self.notebook.add(self.deletion_screen, text="Exclusão em Massa")

    def reload_references(self):
        self.wizard_screen.reload_references()
        self.venue_screen.reload_references()
        self.player_screen.reload_references()
        self.roster_screen.reload_references()
        self.manager_screen.reload_references()
        self.referee_screen.reload_references()
        self.deletion_screen.reload_references()
