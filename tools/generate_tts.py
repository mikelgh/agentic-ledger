#!/usr/bin/env python3
"""Generate TTS audio for demo video scenes using MiniMax API."""
import requests
import json
import os
import base64
import time

# Config
API_KEY = os.environ.get("MINIMAX_API_KEY")
BASE_URL = os.environ.get("MINIMAX_BASE_URL", "https://api.minimax.cn")
VOICE_ID = "Chinese (Mandarin)_News_Anchor"  # 新闻女声
MODEL = "speech-2.6-turbo"  # 快速版
OUTPUT_DIR = r"c:\DevOps\gosim2026-agentic\docs\audio"

# Scene narrations (matched to scene durations)
SCENES = [
    {
        "id": "01_title",
        "text": "Agentic Mail，智能邮件助手，让邮件自动行动。",
    },
    {
        "id": "02_welcome",
        "text": "智能连接邮箱，自动识别服务商，无需手动配置，即可开始使用。",
    },
    {
        "id": "03_inbox",
        "text": "收件箱自动识别邮件类型：日程变更、会议邀请、物流追踪、促销通知，一目了然。",
    },
    {
        "id": "04_schedule",
        "text": "日程变更检测：新旧时间对比，原文来源引用，自动回复草稿，让沟通更高效。",
    },
    {
        "id": "05_reply",
        "text": "回复草稿可编辑，支持一键批准发送或设置自动批准规则。",
    },
    {
        "id": "06_meeting",
        "text": "会议邀请智能提取日期、时间、地点，精准识别关键信息。",
    },
    {
        "id": "07_order",
        "text": "订单物流自动追踪，提取运单号，可视化配送进度，预估到达时间。",
    },
    {
        "id": "08_summary",
        "text": "三大核心能力：日程变更检测、智能日程提取、订单物流追踪，让邮件处理更智能。",
    },
    {
        "id": "09_end",
        "text": "Agentic Mail，让邮件自动行动。",
    },
]


def generate_tts(scene_id, text):
    """Generate TTS audio for a single scene."""
    url = f"{BASE_URL}/v1/t2a_v2"
    headers = {
        "Authorization": f"Bearer {API_KEY}",
        "Content-Type": "application/json"
    }
    payload = {
        "model": MODEL,
        "text": text,
        "stream": False,
        "voice_setting": {
            "voice_id": VOICE_ID,
            "speed": 0.85,  # 稍慢，更清晰
            "vol": 10,
            "pitch": 0
        },
        "audio_setting": {
            "sample_rate": 32000,
            "bitrate": 128000,
            "format": "mp3",
            "channel": 1
        },
        "subtitle_enable": True
    }
    
    print(f"  Generating TTS for {scene_id}...")
    resp = requests.post(url, headers=headers, json=payload, timeout=60)
    
    if resp.status_code != 200:
        print(f"  ERROR: HTTP {resp.status_code}: {resp.text[:200]}")
        return None
    
    data = resp.json()
    
    if data.get("base_resp", {}).get("status_code", -1) != 0:
        print(f"  ERROR: API error: {data.get('base_resp')}")
        return None
    
    # Decode audio
    audio_hex = data["data"]["audio"]
    audio_bytes = bytes.fromhex(audio_hex)
    
    # Save
    out_path = os.path.join(OUTPUT_DIR, f"{scene_id}.mp3")
    with open(out_path, "wb") as f:
        f.write(audio_bytes)
    
    extra = data.get("extra_info", {})
    duration_ms = extra.get("audio_length", 0)
    print(f"  OK: {out_path} ({duration_ms}ms, {len(audio_bytes)} bytes)")
    
    return {
        "id": scene_id,
        "path": out_path,
        "duration_ms": duration_ms,
        "text": text
    }


def main():
    if not API_KEY:
        print("ERROR: MINIMAX_API_KEY not set")
        return
    
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    results = []
    for scene in SCENES:
        result = generate_tts(scene["id"], scene["text"])
        if result:
            results.append(result)
        time.sleep(0.5)  # Rate limit
    
    # Save manifest
    manifest_path = os.path.join(OUTPUT_DIR, "manifest.json")
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    
    print(f"\nDone! Generated {len(results)} audio files.")
    print(f"Manifest: {manifest_path}")


if __name__ == "__main__":
    main()
