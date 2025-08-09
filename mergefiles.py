import os
import re
import sys
import time
from datetime import datetime
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

# Configuration
# Entry point: lib.rs or main.rs
ENTRY_FILES = ['src/lib.rs', 'src/main.rs']
OUTPUT_FILE = 'output.rs'
TEST_FILE = 'test.rs'
WATCH_PATH = 'src'

mod_pattern = re.compile(r'^(?P<indent>\s*)mod\s+(?P<name>\w+)\s*;\s*$', re.MULTILINE)


def find_entry():
    for f in ENTRY_FILES:
        if os.path.isfile(f):
            return f
    print(f"❌ Erreur : aucun fichier d'entrée trouvé parmi {ENTRY_FILES}")
    sys.exit(1)


def inline_modules(file_path, processed=None):
    """
    Recursively inline modules: replace `mod name;` with `mod name { <content> }`.
    """
    if processed is None:
        processed = set()
    content = ''
    dir_path = os.path.dirname(file_path)
    with open(file_path, 'r', encoding='utf-8') as f:
        text = f.read()

    def replace_mod(match):
        indent = match.group('indent')
        name = match.group('name')
        if name in processed:
            return ''  # avoid duplication
        processed.add(name)
        # locate module file
        mod_rs = os.path.join(dir_path, f"{name}.rs")
        mod_dir_rs = os.path.join(dir_path, name, 'mod.rs')
        if os.path.isfile(mod_rs):
            path = mod_rs
        elif os.path.isfile(mod_dir_rs):
            path = mod_dir_rs
        else:
            print(f"⚠️  Module {name} non trouvé. Skip.")
            return match.group(0)
        # read and inline
        inner = inline_modules(path, processed)
        # indent inner content
        indented = '\n'.join(indent + '    ' + line for line in inner.splitlines())
        return f"{indent}mod {name} {{\n{indented}\n{indent}}}"

    # replace all mod declarations
    result = mod_pattern.sub(replace_mod, text)
    return result


def generate():
    entry = find_entry()
    nowHour = datetime.now().strftime("%H:%M:%S")
    now = datetime.now().strftime("%H:%M:%S le %d-%m-%Y")
    print(f"🔄 Génération du fichier fusionné depuis {entry}... à {nowHour}")
    merged = inline_modules(entry)
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as out:
        out.write(f"// Généré à {now}\n")
        out.write(merged)
    print(f"✅  Fichier écrit : {OUTPUT_FILE}")


class ChangeHandler(FileSystemEventHandler):
    def __init__(self, callback):
        super().__init__()
        self.callback = callback

    def on_any_event(self, event):
        if event.is_directory:
            return
        if not event.src_path.endswith('.rs'):
            return
        if os.path.basename(event.src_path) == OUTPUT_FILE:
            return
        if os.path.basename(event.src_path) == TEST_FILE:
                    return
        print(f"⚙️ Changements détectés dans {event.src_path}, régénération...")
        self.callback()


def watch():
    event_handler = ChangeHandler(generate)
    observer = Observer()
    observer.schedule(event_handler, WATCH_PATH, recursive=True)
    observer.start()
    print(f"👀 Surveillance de '{WATCH_PATH}' pour les changements. Ctrl+C pour stopper.")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()


if __name__ == '__main__':
    generate()
    watch()
