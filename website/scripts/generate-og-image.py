#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
from typing import Iterable

from PIL import Image, ImageDraw, ImageFilter, ImageFont

WIDTH = 1200
HEIGHT = 630
BG = '#0b0d13'
SURFACE = '#10131b'
SURFACE_BORDER = '#222834'
TEXT = '#f5f7fb'
MUTED = '#a7afbf'
MUTED_2 = '#778092'
ACCENT = '#3b82f6'
ACCENT_SOFT = '#60a5fa'
SUCCESS = '#22c55e'
MONO_BG = '#121723'
CHIP_BG = '#171c28'
CHIP_BORDER = '#2a3240'

ROOT = Path(__file__).resolve().parents[1]
OUT_PATH = ROOT / 'docs' / 'public' / 'og-image-v2.png'


def load_font(size: int, *, mono: bool = False, bold: bool = False) -> ImageFont.FreeTypeFont | ImageFont.ImageFont:
    candidates: list[str] = []
    if mono:
        candidates.extend([
            '/System/Library/Fonts/SFNSMono.ttf',
            '/System/Library/Fonts/Supplemental/Menlo.ttc',
            '/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf',
            '/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf',
        ])
    else:
        if bold:
            candidates.extend([
                '/System/Library/Fonts/Supplemental/Arial Bold.ttf',
                '/System/Library/Fonts/Supplemental/Helvetica.ttc',
                '/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf',
                '/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf',
            ])
        else:
            candidates.extend([
                '/System/Library/Fonts/Supplemental/Arial.ttf',
                '/System/Library/Fonts/Helvetica.ttc',
                '/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',
                '/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf',
            ])

    for candidate in candidates:
        path = Path(candidate)
        if path.exists():
            try:
                return ImageFont.truetype(str(path), size)
            except Exception:
                continue
    return ImageFont.load_default()


def rounded_panel(img: Image.Image, box: tuple[int, int, int, int], *, fill: str, outline: str | None = None, width: int = 1) -> None:
    draw = ImageDraw.Draw(img)
    draw.rounded_rectangle(box, radius=24, fill=fill, outline=outline, width=width)


def draw_grid(draw: ImageDraw.ImageDraw, spacing: int = 48) -> None:
    for x in range(0, WIDTH, spacing):
        draw.line([(x, 0), (x, HEIGHT)], fill=(255, 255, 255, 14), width=1)
    for y in range(0, HEIGHT, spacing):
        draw.line([(0, y), (WIDTH, y)], fill=(255, 255, 255, 14), width=1)


def wrap_text(text: str, font: ImageFont.ImageFont, max_width: int) -> list[str]:
    measure = ImageDraw.Draw(Image.new('RGB', (10, 10)))
    words = text.split()
    lines: list[str] = []
    current = ''
    for word in words:
        candidate = word if not current else f'{current} {word}'
        width = measure.textbbox((0, 0), candidate, font=font)[2]
        if width <= max_width:
            current = candidate
        else:
            if current:
                lines.append(current)
            current = word
    if current:
        lines.append(current)
    return lines


def draw_badge(draw: ImageDraw.ImageDraw, x: int, y: int, label: str, font: ImageFont.ImageFont) -> int:
    left, top, right, bottom = draw.textbbox((0, 0), label, font=font)
    width = (right - left) + 28
    height = 34
    draw.rounded_rectangle((x, y, x + width, y + height), radius=10, fill=CHIP_BG, outline=CHIP_BORDER, width=1)
    draw.text((x + 14, y + 7), label, font=font, fill='#d9deea')
    return width


def draw_segmented_text(draw: ImageDraw.ImageDraw, x: int, y: int, segments: Iterable[tuple[str, str]], font: ImageFont.ImageFont) -> None:
    cursor_x = x
    for text, color in segments:
        draw.text((cursor_x, y), text, font=font, fill=color)
        bbox = draw.textbbox((cursor_x, y), text, font=font)
        cursor_x = bbox[2]


def main() -> None:
    image = Image.new('RGBA', (WIDTH, HEIGHT), BG)
    draw = ImageDraw.Draw(image)

    # Background grid and glow
    draw_grid(draw)

    glow = Image.new('RGBA', (WIDTH, HEIGHT), (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow)
    glow_draw.ellipse((-80, -120, 760, 520), fill=(59, 130, 246, 62))
    glow_draw.ellipse((360, 120, 1160, 760), fill=(16, 185, 129, 34))
    glow = glow.filter(ImageFilter.GaussianBlur(80))
    image = Image.alpha_composite(image, glow)
    draw = ImageDraw.Draw(image)

    title_font = load_font(74, bold=True)
    subtitle_font = load_font(29)
    body_font = load_font(22)
    pill_font = load_font(22)
    badge_font = load_font(18, mono=True)
    footer_font = load_font(20, bold=True)
    footer_small = load_font(18)
    code_font = load_font(18, mono=True)

    # Brand pill
    pill_x, pill_y = 80, 72
    pill_w, pill_h = 150, 30
    draw.rounded_rectangle((pill_x, pill_y, pill_x + pill_w, pill_y + pill_h), radius=15, fill=(16, 19, 27, 214), outline=(255, 255, 255, 42), width=1)
    draw.ellipse((pill_x + 14, pill_y + 11, pill_x + 22, pill_y + 19), fill=SUCCESS)
    draw.text((pill_x + 30, pill_y + 6), 'meshlang.dev', font=pill_font, fill='#d7dbe5')

    # Left column copy
    title = 'Built for distributed\nsystems.'
    draw.multiline_text((80, 132), title, font=title_font, fill=TEXT, spacing=6)

    subtitle = 'One public app URL. Server-side runtime placement. Operator truth stays on meshc cluster.'
    subtitle_lines = wrap_text(subtitle, subtitle_font, 540)
    subtitle_y = 350
    for i, line in enumerate(subtitle_lines):
        draw.text((80, subtitle_y + i * 38), line, font=subtitle_font, fill=MUTED)

    badge_y = 462
    badge_x = 80
    for badge in ['@cluster', 'LLVM native', 'Type-safe', 'One public URL']:
        badge_x += draw_badge(draw, badge_x, badge_y, badge, badge_font) + 12

    # Code panel
    panel = Image.new('RGBA', (410, 300), (0, 0, 0, 0))
    rounded_panel(panel, (0, 0, 410, 300), fill=(16, 19, 27, 235), outline=(255, 255, 255, 26), width=1)
    panel_draw = ImageDraw.Draw(panel)
    panel_draw.rounded_rectangle((0, 0, 410, 300), radius=24, outline=(255, 255, 255, 36), width=1)
    panel_x, panel_y = 760, 150
    image.alpha_composite(panel, (panel_x, panel_y))
    draw = ImageDraw.Draw(image)

    code_x = panel_x + 24
    code_y = panel_y + 28
    line_gap = 29
    line = 0
    code_lines = [
        [('# One annotation to distribute', MUTED_2)],
        [('@cluster', ACCENT_SOFT), (' pub fn ', '#c6cbda'), ('process_order', '#c084fc'), ('(id) do', '#c6cbda')],
        [('let ', '#c6cbda'), ('order', '#d7dbe5'), (' = Repo.find(pool, id)', '#9aa4b7')],
        [('Payment.charge(order)', '#d7dbe5')],
        [('end', '#c6cbda')],
        [],
        [('# Runtime-owned failover', MUTED_2)],
        [('let key = "order-42"', '#9aa4b7')],
        [('Continuity.submit(key, process_order)', '#9aa4b7')],
    ]
    for segments in code_lines:
        if not segments:
            line += 1
            continue
        draw_segmented_text(draw, code_x, code_y + line * line_gap, segments, code_font)
        line += 1

    # Footer brand
    footer_y = 570
    draw.rounded_rectangle((80, footer_y, 108, footer_y + 28), radius=7, fill='#f5f7fb')
    draw.text((88, footer_y + 2), 'M', font=footer_font, fill=BG)
    draw.text((120, footer_y + 3), 'Mesh Programming Language', font=footer_font, fill='#d7dbe5')
    draw.text((1028, footer_y + 4), 'meshlang.dev', font=footer_small, fill=MUTED_2)

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    image.convert('RGB').save(OUT_PATH, optimize=True)
    print(f'Wrote {OUT_PATH.relative_to(ROOT)}')


if __name__ == '__main__':
    main()
