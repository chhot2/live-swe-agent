#!/usr/bin/env python3
"""Tool to generate GitHub issues from implementation checklist."""

import subprocess
import sys
import re

def parse_checklist(filepath):
    """Parse the checklist file and extract uncompleted tasks organized by phase."""
    phases = {}
    current_phase = None
    current_section = None
    
    with open(filepath, 'r') as f:
        lines = f.readlines()
    
    for line in lines:
        # Detect phase headers (## Phase X: ...)
        phase_match = re.match(r'^## (Phase \d+:.+?)(?:\s*[⚙️🌐🔌🎨🔒📦✨🚀])*\s*$', line.strip())
        if phase_match:
            current_phase = phase_match.group(1).strip()
            phases[current_phase] = {}
            continue
        
        # Detect section headers (### ...)
        section_match = re.match(r'^### (.+)$', line.strip())
        if section_match and current_phase:
            current_section = section_match.group(1).strip()
            phases[current_phase][current_section] = []
            continue
        
        # Detect uncompleted tasks (- [ ] ...)
        task_match = re.match(r'^- \[ \] (.+)$', line.strip())
        if task_match and current_phase and current_section:
            task = task_match.group(1).strip()
            phases[current_phase][current_section].append(task)
    
    return phases

def generate_issue_content(phase, section, tasks):
    """Generate issue title and body."""
    title = f"[{phase.split(':')[0]}] {section}"
    
    body = f"""## Description
Implement the following tasks for **{section}** in **{phase}**.

## Tasks
"""
    for task in tasks:
        body += f"- [ ] {task}\n"
    
    body += f"""
## Context
This issue is part of the implementation checklist for the Proxy Desktop Browser project.

## Acceptance Criteria
- All tasks listed above are completed
- Code is tested and working
- Documentation is updated if needed
"""
    return title, body

def create_github_issue(title, body, labels=None):
    """Create a GitHub issue using gh cli."""
    cmd = ['gh', 'issue', 'create', '--title', title, '--body', body]
    if labels:
        cmd.extend(['--label', ','.join(labels)])
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode == 0:
        print(f"✅ Created issue: {title}")
        print(f"   URL: {result.stdout.strip()}")
        return True
    else:
        print(f"❌ Failed to create issue: {title}")
        print(f"   Error: {result.stderr}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python gh_issue_generator.py <checklist_file> [--create] [--phase <phase_num>]")
        print("\nOptions:")
        print("  --create     Actually create the issues on GitHub")
        print("  --phase N    Only process phase N (e.g., --phase 3)")
        print("  --list       List all phases and sections with task counts")
        sys.exit(1)
    
    filepath = sys.argv[1]
    create_issues = '--create' in sys.argv
    list_only = '--list' in sys.argv
    
    phase_filter = None
    if '--phase' in sys.argv:
        idx = sys.argv.index('--phase')
        if idx + 1 < len(sys.argv):
            phase_filter = sys.argv[idx + 1]
    
    phases = parse_checklist(filepath)
    
    if list_only:
        print("\n=== Implementation Checklist Summary ===\n")
        total_tasks = 0
        for phase, sections in phases.items():
            phase_tasks = sum(len(tasks) for tasks in sections.values())
            total_tasks += phase_tasks
            print(f"\n{phase} ({phase_tasks} tasks)")
            for section, tasks in sections.items():
                if tasks:
                    print(f"  - {section}: {len(tasks)} tasks")
        print(f"\n{'='*40}")
        print(f"Total uncompleted tasks: {total_tasks}")
        return
    
    print("\n=== GitHub Issue Generator ===\n")
    
    issues_to_create = []
    
    for phase, sections in phases.items():
        if phase_filter and not phase.startswith(f"Phase {phase_filter}"):
            continue
            
        for section, tasks in sections.items():
            if tasks:  # Only create issues for sections with uncompleted tasks
                title, body = generate_issue_content(phase, section, tasks)
                issues_to_create.append((title, body, phase))
    
    print(f"Found {len(issues_to_create)} potential issues to create:\n")
    
    for i, (title, body, phase) in enumerate(issues_to_create, 1):
        print(f"{i}. {title}")
    
    if create_issues:
        print(f"\nCreating {len(issues_to_create)} issues on GitHub...\n")
        for title, body, phase in issues_to_create:
            create_github_issue(title, body)
    else:
        print("\nRun with --create flag to actually create these issues on GitHub.")
        print("Example: python gh_issue_generator.py IMPLEMENTATION_CHECKLIST.md --create")

if __name__ == "__main__":
    main()
