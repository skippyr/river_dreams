#!/usr/bin/env zsh

river_dreams::git::get_current_branch() {
  git branch --show-current 2>/dev/null
}

river_dreams::git::get_commit_hash() {
  git rev-parse --short HEAD 2>/dev/null
}

river_dreams::git::get_changes_quantity() {
  git status -s | grep "[?NMDT] \S" | wc -l
}

river_dreams::git::get_staged_quantity() {
  git diff --name-only --cached | wc -l
}

river_dreams::git::get_diff() {
  git branch -v |
  grep "* " |
  awk -F '[][]' '{print $2}'
}

river_dreams::git::get_diff_status() {
  local -r diff="$1"
  [[ $(echo "${diff}" | grep ",") ]] &&
    echo "both" ||
    echo "${diff}" | cut -f 1 -d " "
}

river_dreams::git::get_pushes_quantity() {
  local -r diff="$1"
  local -r diff_status="$2"
  case ${diff_status} in
    both)
      echo "${diff}" | cut -f 2 -d " " | tr -d ","
      ;;
    ahead)
      echo "${diff}" | cut -f 2 -d " "
      ;;
    *)
      echo 0
      ;;
  esac
}

river_dreams::git::get_pulls_quantity() {
  local -r diff="$1"
  local -r diff_status="$2"
  case ${diff_status} in
    both)
      echo "${diff}" | cut -f 4 -d " " 
      ;;
    behind)
      echo "${diff}" | cut -f 2 -d " "
      ;;
    *)
      echo 0
      ;;
  esac
}

river_dreams::git() {
  local -r current_branch=$(river_dreams::git::get_current_branch)
  if [[ -n ${current_branch} ]]; then
    local commit_hash=$(river_dreams::git::get_commit_hash)
    [[ -n ${commit_hash} ]] && commit_hash=" ${commit_hash}"
    
    local changes_quantity=$(river_dreams::git::get_changes_quantity)
    local changes_section=""
    [[ ${changes_quantity} -gt 0 ]] && changes_section="%F{red}${changes_quantity}*%f"
    
    local staged_quantity=$(river_dreams::git::get_staged_quantity)
    local staged_section=""
    [[ ${staged_quantity} -gt 0 ]] && staged_section="%F{green}${staged_quantity}+%f"

    local -r diff=$(river_dreams::git::get_diff)
    local -r diff_status=$(river_dreams::git::get_diff_status "${diff}")
    local -r diff_pushes_quantity=$(river_dreams::git::get_pushes_quantity "${diff}" "${diff_status}")
    local -r diff_pulls_quantity=$(river_dreams::git::get_pulls_quantity "${diff}" "${diff_status}")
    local -r diff_pushes_symbol=$(
      [[ ${RIVER_DREAMS_USE_FALLBACK_TEXT} == true ]] &&
        echo "^" ||
        echo "↑"
    )
    local -r diff_pulls_symbol=$(
      [[ ${RIVER_DREAMS_USE_FALLBACK_TEXT} == true ]] &&
        echo "v" ||
        echo "↓"
    )
    local diff_pushes_section=""
    local diff_pulls_section=""
    [[ ${diff_pushes_quantity} -gt 0 ]] && diff_pushes_section="%F{yellow}${diff_pushes_quantity}${diff_pushes_symbol}%f"
    [[ ${diff_pulls_quantity} -gt 0 ]] && diff_pulls_section="%F{blue}${diff_pulls_quantity}${diff_pulls_symbol}%f"
    local diff_section=(
      ${diff_pushes_section}
      ${diff_pulls_section}
    )

    local status_section=(
      ${changes_section}
      ${staged_section}
      ${diff_section}
    )
    [[ -n ${status_section} ]] && status_section=" [${status_section}]"

    local tag=$(git describe --tags --abbrev=0 2>/dev/null)
    [[ -n ${tag} ]] && tag=" ${tag}"

    local -r branches_quantity=$(git branch -a | cut -f 3 -d / | grep -v HEAD | tr -d " " | tr -d "*" | sort | uniq | wc -l)
    local branches_section=""
    [[ ${branches_quantity} -gt 1 ]] && branches_section="${branches_quantity}@ "

    local -r main_branch=$(git branch -a | grep HEAD | cut -f 2 -d ">" | cut -f 2 -d "/")
    local current_branch_section="${current_branch}"
    [[ ${main_branch} == ${current_branch} ]] && current_branch_section="${current_branch}%F{yellow}%%%f"

    echo "%F{red}::«%F{cyan}${branches_section}%f%B${current_branch_section}%b${status_section}%F{magenta}${tag}%F{yellow}${commit_hash}%F{red}»"
  fi
}
