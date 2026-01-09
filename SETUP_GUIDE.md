# Live-SWE-Agent Setup Guide

## Prerequisites
✅ Repository cloned: `/root/live-swe-agent`
✅ Virtual environment created and dependencies installed
✅ Copilot API Proxy running on `http://localhost:4141/v1`

## Quick Start

### 1. Single Instance Test
```bash
cd /root/live-swe-agent
source venv/bin/activate
MSWEA_COST_TRACKING='ignore_errors' \
OPENAI_API_KEY="sk-dummy" \
OPENAI_API_BASE="http://localhost:4141/v1" \
mini -c config/livesweagent.yaml \
     -m openai/claude-opus-4.5 \
     -t "Your task description here" \
     -y
```

### 2. Parallel Execution (4 instances)
Use the provided helper script:
```bash
cd /root/live-swe-agent
./run_parallel.sh
```

Or run manually:
```bash
cd /root/live-swe-agent && source venv/bin/activate && \
for i in 1 2 3 4; do \
  MSWEA_COST_TRACKING='ignore_errors' \
  OPENAI_API_KEY="sk-dummy" \
  OPENAI_API_BASE="http://localhost:4141/v1" \
  mini -c config/livesweagent.yaml \
       -m openai/claude-opus-4.5 \
       -t "hello" \
       -y & \
done && wait
```

## Available Models
The copilot API proxy provides access to:
- `openai/claude-opus-4.5` (recommended for best performance)
- `openai/claude-sonnet-4.5`
- `openai/gpt-5.1-codex-max`
- `openai/gpt-5.2`
- `openai/gpt-4.1`
- `openai/gemini-3-pro-preview`

## Configuration Files
- `config/livesweagent.yaml` - Standard Live-SWE-agent config
- `config/livesweagent_swebench.yaml` - SWE-bench Verified config
- `config/livesweagent_swebench_pro.yaml` - SWE-Bench Pro config

## Environment Variables
- `MSWEA_COST_TRACKING='ignore_errors'` - Ignore cost tracking errors
- `OPENAI_API_KEY="sk-dummy"` - Dummy key for copilot proxy
- `OPENAI_API_BASE="http://localhost:4141/v1"` - Copilot API endpoint

## Customization
To change the task, modify the `-t` parameter:
```bash
mini -c config/livesweagent.yaml \
     -m openai/claude-opus-4.5 \
     -t "Fix the bug in login.py where users can't authenticate" \
     -y
```

## Rate Limiting
The copilot API proxy is configured with:
- Rate limit: 4 concurrent requests
- Wait mode enabled

## Troubleshooting
1. **API not responding**: Ensure copilot proxy is running:
   ```bash
   curl http://localhost:4141/v1/models
   ```

2. **Virtual environment issues**: Recreate venv:
   ```bash
   cd /root/live-swe-agent
   rm -rf venv
   python3 -m venv venv
   source venv/bin/activate
   pip install mini-swe-agent
   ```

3. **Permission errors**: Make script executable:
   ```bash
   chmod +x run_parallel.sh
   ```
