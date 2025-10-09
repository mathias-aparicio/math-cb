default: html

html:
    @mkdir -p frontend/html/cours
    @mkdir -p frontend/html/exercices
    @for src_file in frontend/typst/cours/*.typ frontend/typst/exercices/*.typ; do \
        if [ -f "$src_file" ]; then \
            category=$(basename $(dirname "$src_file")); \
            filename=$(basename "$src_file" .typ); \
            dest_file="frontend/html/$category/$filename.html"; \
            if [ ! -f "$dest_file" ] || [ "$src_file" -nt "$dest_file" ]; then \
                echo "  Compiling $src_file"; \
                typst c "$src_file" --format html --features html "$dest_file"; \
            fi \
        fi \
    done

run: html
    @cd backend && cargo run

clean:
    @rm -f frontend/html/cours/*.html
    @rm -f frontend/html/exercices/*.html
