-- Neovim LSP Configuration for Kora Language Server (kora lsp)
-- Add this file to your Neovim configuration (e.g. ~/.config/nvim/after/ftplugin/kora.lua
-- or include it in your init.lua / lspconfig setup).

local client_name = 'kora_lsp'

local function start_kora_lsp()
  local root_dir = vim.fs.dirname(
    vim.fs.find({ 'kora.toml', '.git' }, { upward = true, path = vim.api.nvim_buf_get_name(0) })[1]
  ) or vim.fn.getcwd()

  vim.lsp.start({
    name = client_name,
    cmd = { 'kora', 'lsp' },
    root_dir = root_dir,
    filetypes = { 'kora' },
    settings = {},
  })
end

-- Automatically start LSP when a Kora file is opened
vim.api.nvim_create_autocmd('FileType', {
  pattern = 'kora',
  callback = start_kora_lsp,
})

