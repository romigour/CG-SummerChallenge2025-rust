import os
import time
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
from datetime import datetime

# Configuration
OUTPUT_FILE = "bot_unified.rs"
PROJECT_ROOT = "."  # Répertoire racine du projet

def collect_rs_files(root_dir):
    rs_files = []
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith(".rs") and file != OUTPUT_FILE:
                full_path = os.path.join(root, file)
                rs_files.append(full_path)

    # Prioriser main.rs s'il existe
    rs_files.sort(key=lambda x: (os.path.basename(x) != "main.rs", x))
    return rs_files

def unify_files(rs_files, output_path):
    with open(output_path, "w", encoding="utf-8") as outfile:
        # Ajouter l'en-tête avec date et heure
        now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        outfile.write(f"// === Fichier unifié automatiquement ===\n")
        outfile.write(f"// Date de merge : {now}\n")
        outfile.write(f"// Nombre de fichiers .rs : {len(rs_files)}\n\n")

        for file_path in rs_files:
            outfile.write(f"\n// --- Début du fichier: {file_path} ---\n")
            with open(file_path, "r", encoding="utf-8") as infile:
                content = infile.read()
                outfile.write(content)
                outfile.write(f"\n// --- Fin du fichier: {file_path} ---\n")
    print(f"[✓] Unification terminée : {output_path} ({len(rs_files)} fichiers)")



# def unify_files(rs_files, output_path):
#     with open(output_path, "w", encoding="utf-8") as outfile:
#         for file_path in rs_files:
#             outfile.write(f"\n// --- File: {file_path} ---\n")
#             with open(file_path, "r", encoding="utf-8") as infile:
#                 content = infile.read()
#                 outfile.write(content)
#                 outfile.write("\n")
#     print(f"[✓] Unification terminée : {output_path} ({len(rs_files)} fichiers)")

class RsFileChangeHandler(FileSystemEventHandler):
    def on_modified(self, event):
        if event.src_path.endswith(".rs") and not event.src_path.endswith(OUTPUT_FILE):
            print(f"[!] Changement détecté : {event.src_path}")
            rs_files = collect_rs_files(PROJECT_ROOT)
            unify_files(rs_files, os.path.join(PROJECT_ROOT, OUTPUT_FILE))

    def on_created(self, event):
        self.on_modified(event)

    def on_deleted(self, event):
        self.on_modified(event)

def watch_and_unify():
    rs_files = collect_rs_files(PROJECT_ROOT)
    unify_files(rs_files, os.path.join(PROJECT_ROOT, OUTPUT_FILE))

    event_handler = RsFileChangeHandler()
    observer = Observer()
    observer.schedule(event_handler, path=PROJECT_ROOT, recursive=True)
    observer.start()
    print("[👀] Surveillance des fichiers .rs lancée. Appuyez sur Ctrl+C pour arrêter.")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()

if __name__ == "__main__":
    watch_and_unify()
