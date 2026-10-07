#!/usr/bin/env python3
"""Assemble final demo video with audio and subtitles."""
import json
import os
import subprocess
from PIL import Image, ImageDraw, ImageFont
import numpy as np

# Paths
AUDIO_DIR = r"c:\DevOps\gosim2026-agentic\docs\audio"
SCREENSHOT_DIR = r"c:\DevOps\gosim2026-agentic\docs\screenshots"
OUTPUT_VIDEO = r"c:\DevOps\gosim2026-agentic\docs\agentic-mail-demo-final.mp4"
TEMP_DIR = r"c:\DevOps\gosim2026-agentic\docs\temp_frames"

# Video settings
FPS = 30
WIDTH = 1080
HEIGHT = 1920
PADDING = 5.0  # seconds of silence after each narration

# Load audio manifest
with open(os.path.join(AUDIO_DIR, "manifest.json"), "r", encoding="utf-8") as f:
    audio_manifest = json.load(f)

# Scene definitions (screenshot + audio)
SCENES = [
    {"type": "title", "audio": "01_title.mp3", "subtitle": "Agentic Mail\n智能邮件助手"},
    {"type": "screenshot", "image": "verify-welcome.png", "audio": "02_welcome.mp3", "subtitle": "智能连接邮箱\n自动识别服务商"},
    {"type": "screenshot", "image": "01-inbox.png", "audio": "03_inbox.mp3", "subtitle": "自动识别邮件类型\n一目了然"},
    {"type": "screenshot", "image": "02-d1-schedule.png", "audio": "04_schedule.mp3", "subtitle": "日程变更检测\n新旧时间对比"},
    {"type": "screenshot", "image": "02-d1-schedule.png", "audio": "05_reply.mp3", "subtitle": "回复草稿可编辑\n支持一键批准发送", "crop_top": 0.55},
    {"type": "screenshot", "image": "03-d2-meeting.png", "audio": "06_meeting.mp3", "subtitle": "会议邀请\n智能提取关键信息"},
    {"type": "screenshot", "image": "04-d3-order.png", "audio": "07_order.mp3", "subtitle": "订单物流自动追踪\n可视化配送进度"},
    {"type": "title", "audio": "08_summary.mp3", "subtitle": "三大核心能力\n让邮件处理更智能", "is_summary": True},
    {"type": "title", "audio": "09_end.mp3", "subtitle": "Agentic Mail\n让邮件自动行动"},
]


def get_audio_duration(audio_path):
    """Get audio duration using ffprobe."""
    cmd = ["ffprobe", "-v", "error", "-show_format", audio_path]
    result = subprocess.run(cmd, capture_output=True, text=True)
    for line in result.stdout.split("\n"):
        if line.startswith("duration="):
            return float(line.split("=")[1])
    return 0.0


def create_title_frame(title, subtitle, is_summary=False, is_end=False):
    """Create a title card frame."""
    img = Image.new("RGB", (WIDTH, HEIGHT), color=(15, 23, 42))
    draw = ImageDraw.Draw(img)
    
    try:
        font_title = ImageFont.truetype("msyh.ttc", 80)
        font_sub = ImageFont.truetype("msyh.ttc", 50)
    except:
        font_title = ImageFont.load_default()
        font_sub = ImageFont.load_default()
    
    # Title
    bbox = draw.textbbox((0, 0), title, font=font_title)
    tw = bbox[2] - bbox[0]
    tx = (WIDTH - tw) // 2
    ty = HEIGHT // 3
    draw.text((tx, ty), title, fill=(255, 255, 255), font=font_title)
    
    # Subtitle
    if is_summary:
        # Show capability list
        items = ["• 日程变更检测", "• 智能日程提取", "• 订单物流追踪"]
        y = ty + 150
        for item in items:
            bbox = draw.textbbox((0, 0), item, font=font_sub)
            tw = bbox[2] - bbox[0]
            tx = (WIDTH - tw) // 2
            draw.text((tx, y), item, fill=(100, 200, 255), font=font_sub)
            y += 80
    elif is_end:
        # End card
        bbox = draw.textbbox((0, 0), subtitle, font=font_sub)
        tw = bbox[2] - bbox[0]
        tx = (WIDTH - tw) // 2
        ty2 = ty + 150
        draw.text((tx, ty2), subtitle, fill=(200, 200, 200), font=font_sub)
    else:
        bbox = draw.textbbox((0, 0), subtitle, font=font_sub)
        tw = bbox[2] - bbox[0]
        tx = (WIDTH - tw) // 2
        ty2 = ty + 150
        draw.text((tx, ty2), subtitle, fill=(200, 200, 200), font=font_sub)
    
    return img


def create_screenshot_frame(image_path, subtitle, crop_top=0):
    """Create a screenshot frame with optional crop."""
    img = Image.open(os.path.join(SCREENSHOT_DIR, image_path)).convert("RGB")
    
    # Crop if needed
    if crop_top > 0:
        h = img.height
        top = int(h * crop_top)
        img = img.crop((0, top, img.width, h))
    
    # Resize to fit
    scale = min(WIDTH / img.width, HEIGHT / img.height) * 0.9
    new_w = int(img.width * scale)
    new_h = int(img.height * scale)
    img = img.resize((new_w, new_h), Image.Resampling.LANCZOS)
    
    # Center on background
    bg = Image.new("RGB", (WIDTH, HEIGHT), color=(15, 23, 42))
    x = (WIDTH - new_w) // 2
    y = (HEIGHT - new_h) // 2 - 100
    bg.paste(img, (x, y))
    
    # Add subtitle
    draw = ImageDraw.Draw(bg)
    try:
        font = ImageFont.truetype("msyh.ttc", 45)
    except:
        font = ImageFont.load_default()
    
    lines = subtitle.split("\n")
    y_pos = HEIGHT - 300
    for line in lines:
        bbox = draw.textbbox((0, 0), line, font=font)
        tw = bbox[2] - bbox[0]
        tx = (WIDTH - tw) // 2
        draw.text((tx, y_pos), line, fill=(255, 255, 255), font=font)
        y_pos += 70
    
    return bg


def main():
    os.makedirs(TEMP_DIR, exist_ok=True)
    
    # Calculate scene durations based on audio
    scene_data = []
    for i, scene in enumerate(SCENES):
        audio_path = os.path.join(AUDIO_DIR, scene["audio"])
        duration = get_audio_duration(audio_path) + PADDING
        scene_data.append({
            **scene,
            "duration": duration,
            "audio_path": audio_path,
            "index": i
        })
        print(f"Scene {i+1}: {duration:.1f}s (audio: {duration - PADDING:.1f}s)")
    
    total_duration = sum(s["duration"] for s in scene_data)
    print(f"\nTotal duration: {total_duration:.1f}s")
    
    # Generate frames
    print("\nGenerating frames...")
    frame_files = []
    frame_idx = 0
    
    for scene in scene_data:
        num_frames = int(scene["duration"] * FPS)
        
        # Create base frame
        if scene["type"] == "title":
            is_summary = scene.get("is_summary", False)
            is_end = scene["index"] == len(SCENES) - 1
            frame = create_title_frame(scene["subtitle"].split("\n")[0], scene["subtitle"], is_summary, is_end)
        else:
            frame = create_screenshot_frame(scene["image"], scene["subtitle"], scene.get("crop_top", 0))
        
        # Save frames
        for _ in range(num_frames):
            frame_path = os.path.join(TEMP_DIR, f"frame_{frame_idx:05d}.png")
            frame.save(frame_path)
            frame_files.append(frame_path)
            frame_idx += 1
    
    print(f"Generated {len(frame_files)} frames")
    
    # Create video from frames
    print("\nCreating video from frames...")
    temp_video = os.path.join(TEMP_DIR, "temp_video.mp4")
    cmd = [
        "ffmpeg", "-y",
        "-framerate", str(FPS),
        "-i", os.path.join(TEMP_DIR, "frame_%05d.png"),
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-preset", "medium",
        "-crf", "23",
        temp_video
    ]
    subprocess.run(cmd, capture_output=True)
    
    # Concatenate audio with silence padding to sync with video scenes
    print("\nConcatenating audio with silence padding...")
    # Generate silence file matching scene padding (32000 Hz mono, same as TTS output)
    silence_path = os.path.join(TEMP_DIR, "silence.mp3")
    cmd = [
        "ffmpeg", "-y",
        "-f", "lavfi", "-i", f"anullsrc=r=32000:cl=mono",
        "-t", str(PADDING),
        "-c:a", "libmp3lame", "-b:a", "128k",
        silence_path
    ]
    subprocess.run(cmd, capture_output=True)
    
    # Create concat file: each scene audio followed by silence
    concat_file = os.path.join(TEMP_DIR, "audio_concat.txt")
    with open(concat_file, "w", encoding="utf-8") as f:
        for scene in scene_data:
            f.write(f"file '{scene['audio_path']}'\n")
            f.write(f"file '{silence_path}'\n")
    
    temp_audio = os.path.join(TEMP_DIR, "temp_audio.mp3")
    cmd = [
        "ffmpeg", "-y",
        "-f", "concat",
        "-safe", "0",
        "-i", concat_file,
        "-c:a", "libmp3lame", "-b:a", "128k",
        temp_audio
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"ERROR audio concat: {result.stderr[:500]}")
    
    # Combine video and audio
    print("\nCombining video and audio...")
    cmd = [
        "ffmpeg", "-y",
        "-i", temp_video,
        "-i", temp_audio,
        "-c:v", "copy",
        "-c:a", "aac",
        "-b:a", "192k",
        "-map", "0:v:0",
        "-map", "1:a:0",
        OUTPUT_VIDEO
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"ERROR: {result.stderr[:500]}")
    else:
        size = os.path.getsize(OUTPUT_VIDEO)
        print(f"\nDone! Output: {OUTPUT_VIDEO}")
        print(f"Size: {size / (1024*1024):.2f} MB")
    
    # Cleanup temp frames
    print("\nCleaning up temp frames...")
    for f in frame_files:
        if os.path.exists(f):
            os.remove(f)


if __name__ == "__main__":
    main()
