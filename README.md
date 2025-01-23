# Gruvboxer Image Converter

![Gruvbox Palette](https://raw.githubusercontent.com/morhetz/gruvbox-contrib/master/logo.png)  
*A CLI tool for transforming images to the iconic Gruvbox color scheme in RUST*

## Features
- **Accurate Color Matching**  
  Uses CIE Lab color space for perceptual accuracy
- **Style Presets**:
  - `gruvbox`: Base color transformation
  - `retro`: VHS effects + film grain
  - `synthwave`: Neon gradient overlay
  - `mosaic`: Pixel art effect
  - `watercolor`: Painterly texture simulation

## Installation
```bash
git clone https://github.com/longelas/gruvboxer
cd gruvboxer
cargo build --release
```

## Usage
### Basic Command
```bash
./gruvbox_converter <INPUT> [OUTPUT] [STRENGTH] [STYLE]
```

### Examples
```bash
# Natural Gruvbox conversion
./gruvbox_converter input.jpg output.png 0.6 gruvbox

# Retro style with film grain
./gruvbox_converter photo.jpg retro_output.jpg 0.7 retro

# Strong synthwave effect
./gruvbox_converter night.png synthwave.png 0.9 synthwave
```

### Options
| Parameter | Description          | Default     | Valid Values          |
|-----------|----------------------|-------------|-----------------------|
| `INPUT`   | Input image path     | Required    | JPEG/PNG/BMP/TIFF     |
| `OUTPUT`  | Output image path    | `output.png`| PNG/JPEG             |
| `STRENGTH`| Color intensity      | `0.7`       | `0.0` (none) - `1.0` (full) |
| `STYLE`   | Artistic style       | `gruvbox`   | See presets above     |

## Technical Specs
- **Processing**: Bilateral filtering (σ=2.0) + sigmoid blending
- **Performance**:
  - 1080p: 2-3 seconds
  - 4K: 10-15 seconds
- **Formats**: Input (JPEG/PNG/BMP/TIFF), Output (PNG/JPEG)

## License
MIT License - See [LICENSE](LICENSE)
