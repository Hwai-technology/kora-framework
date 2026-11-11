" Vim syntax file for Kora programming language
" Language: Kora (*.kora, *.kr)
" Maintainer: Hwai Technology / Kora Team
" Latest Revision: 2026

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword koraKeyword fn let var return if else match while for in async await pub module import from as extend constructor
syn keyword koraDeclaration class model interface enum

" Built-in Types
syn keyword koraType int float string bool void Result Option Task Channel Response

" Boolean and Special Constants
syn keyword koraBoolean true false
syn keyword koraConstant null

" Prohibited Tokens (highlight as Error if typed)
syn keyword koraError undefined struct

" Decorators / Annotations (@controller, @service, etc.)
syn match koraDecorator "@[a-zA-Z_][a-zA-Z0-9_]*"

" Numbers
syn match koraNumber "\v<\d+>"
syn match koraFloat  "\v<\d+\.\d+>"

" Strings and Interpolation
syn region koraString start=+"+ skip=+\\"+ end=+"+ contains=koraStringEscape
syn region koraString start=+'+ skip=+\\'+ end=+'+ contains=koraStringEscape
syn match koraStringEscape "\\." contained

" Comments
syn match koraComment "//.*$" contains=@Spell
syn region koraComment start="/\*" end="\*/" contains=@Spell

" Operators
syn match koraOperator "\v\="
syn match koraOperator "\v\+"
syn match koraOperator "\v\-"
syn match koraOperator "\v\*"
syn match koraOperator "\v\/"
syn match koraOperator "\v\=\>"
syn match koraOperator "\v-\>"
syn match koraOperator "\v\:\:"
syn match koraOperator "\v\:"

" Highlight mappings
hi def link koraKeyword Statement
hi def link koraDeclaration Structure
hi def link koraType Type
hi def link koraBoolean Boolean
hi def link koraConstant Constant
hi def link koraError Error
hi def link koraDecorator PreProc
hi def link koraNumber Number
hi def link koraFloat Float
hi def link koraString String
hi def link koraStringEscape Special
hi def link koraComment Comment
hi def link koraOperator Operator

let b:current_syntax = "kora"

