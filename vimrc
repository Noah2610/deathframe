nmap <leader>r :!RUN_NEW_TERMINAL=1 bin/build<CR>
nmap <leader>R :!RUN_NEW_TERMINAL=1 RUST_BACKTRACE=1 bin/build 2>&1 \| grep -A 1 'deathframe'<CR>
nmap <leader>t :!RUN_NEW_TERMINAL=1 bin/test<CR>
