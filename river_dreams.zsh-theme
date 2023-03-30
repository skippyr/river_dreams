#!/usr/bin/env zsh

export VIRTUAL_ENV_DISABLE_PROMPT=1
export RIVER_DREAMS_ROOT_DIRECTORY=$(dirname $0)
typeset -g RIVER_DREAMS_SOURCES_DIRECTORY=${RIVER_DREAMS_ROOT_DIRECTORY}/\
sources
typeset -g RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY=${RIVER_DREAMS_ROOT_DIRECTORY}/\
distributions
export RIVER_DREAMS_USE_FALLBACK_TEXT=${RIVER_DREAMS_USE_FALLBACK_TEXT:-$(
  [[ $(tput colors) -eq 8 ]] &&
    echo 1 ||
    echo 0
)}
export RIVER_DREAMS_USE_COLOR_VARIANTS=${RIVER_DREAMS_USE_COLOR_VARIANTS:-0}

setopt promptsubst
setopt +o nomatch

river_dreams::recompile_modules()
{
  echo "Recompiling modules..."
  rm -rf ${RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY}
  mkdir -p ${RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY}
  for file in $(ls ${RIVER_DREAMS_SOURCES_DIRECTORY}/modules); do
    gcc ${RIVER_DREAMS_SOURCES_DIRECTORY}/modules/${file} -o\
    ${RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY}/$(echo ${file} | cut -f 1 -d ".")
    echo "  \e[31m*\e[0m Recompiled ${file}."
  done
}

if [[
  ! $(ls ${RIVER_DREAMS_SOURCES_DIRECTORY}/modules | wc -l) -eq
  $(ls ${RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY} 2>/dev/null | wc -l)
]]; then
  river_dreams::recompile_modules
fi

river_dreams::execute()
{
  ${RIVER_DREAMS_DISTRIBUTIONS_DIRECTORY}/$@
}

river_dreams::separator()
{
  river_dreams::execute separator ${COLUMNS}
}

river_dreams::disk()
{
  river_dreams::execute disk
}

river_dreams::arrow()
{
  river_dreams::execute arrow
}

river_dreams::directory()
{
  river_dreams::execute directory
}

river_dreams::pyenv() {
  river_dreams::execute pyenv
}

river_dreams::top_prompt()
{
  typeset -ra top_prompt=(
    $(river_dreams::disk)
    $(river_dreams::pyenv)
  )
  echo ${top_prompt}
}

river_dreams::left_prompt()
{
  echo "$(river_dreams::arrow)$(river_dreams::directory)"
}

PROMPT='$(river_dreams::separator)%F{red}┌─%F{yellow}[%f\
$(river_dreams::top_prompt)%F{yellow}]%f
%F{red}└%f$(river_dreams::left_prompt) '

