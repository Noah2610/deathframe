nmap <leader>r :!cargo +nightly build --features amethyst/nightly<CR>
nmap <leader>R :!RUST_BACKTRACE=1 cargo +nightly build --features amethyst/nightly 2>&1 \| grep -A 1 'hello_amethyst_platformer'<CR>
nmap <leader>t :!cargo +nightly test --features amethyst/nightly<CR>
