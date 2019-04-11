nmap <leader>r :!bin/build<CR>
nmap <leader>R :!RUST_BACKTRACE=1 bin/build 2>&1 \| grep -A 1 'deathframe'<CR>
nmap <leader>t :!bin/build test<CR>
