import os
import sys

current_dir = os.path.dirname(os.path.abspath(__file__))
parent_dir = os.path.dirname(current_dir)
if parent_dir not in sys.path:
    sys.path.insert(0, parent_dir)

from arlo_db_tool.ui.app import App

def main():
    app = App()
    app.mainloop()

if __name__ == "__main__":
    main()