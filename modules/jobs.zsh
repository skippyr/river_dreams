#!/usr/bin/env zsh

river_dreams::jobs() {
  local -r jobs_symbol=$(
    [[ ${RIVER_DREAMS_USE_FALLBACK_TEXT} == true ]] &&
    echo "JOBS" ||
    echo ""
  )
  echo "%(1j.%F{green}${jobs_symbol} %f%j.)"
}
