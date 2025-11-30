# Themes System

The nanometers audio visualizer includes a theme system that allows users to customize the appearance of the application.

## Built-in Themes

The following themes are embedded in the application:

### Basic Themes
- **Dark** (default): Dark theme with blue spectrum lines
- **Light**: Clean light theme with purple and orange spectrum lines  
- **Pink**: Pink theme with white and magenta spectrum lines

### Factory Themes
- **Cyberpunk**: Neon cyan and magenta cyberpunk aesthetic
- **Ocean**: Blue and teal ocean-inspired colors
- **Matrix**: Green terminal theme with contrasting red/green spectrum lines

## Color Properties

Each theme defines the following colors:

- `main`: Primary color for main UI elements
- `bg`: Background color
- `bgaccent`: Accent background color for UI elements
- `text`: Text color
- `accent`: Accent color for highlights
- `frame`: Frame and border color
- `selection`: Selection highlight color
- `spectrum_main`: Primary spectrum line color (L/Left/Mid channel)
- `spectrum_secondary`: Secondary spectrum line color (R/Right/Side channel)
- `spectrum_ref_line`: Reference frequency grid lines

## Theme Management

### Creating Themes
1. Click "New" in the Theme settings panel
2. Enter a name for your custom theme
3. Use the color pickers to customize colors
4. Click "Save" to store the theme

### Editing Themes
- **Built-in themes**: Creates a copy with a new name for editing
- **Custom themes**: Edit directly with full color picker interface

### Theme Protection
Built-in themes are protected from modification:
- Cannot be deleted
- Cannot be directly overwritten
- Editing creates an editable copy

## Storage Location

Custom themes are stored in the user's configuration directory:
- **macOS**: `~/Library/Application Support/Nanometers/themes/`
- **Windows**: `%APPDATA%/Nanometers/themes/`
- **Linux**: `~/.local/share/Nanometers/themes/`

## Theme File Format

Custom themes are stored as JSON files:

```json
{
  "name": "Theme Name",
  "main": [r, g, b, a],
  "bg": [r, g, b, a],
  "bgaccent": [r, g, b, a],
  "text": [r, g, b, a],
  "accent": [r, g, b, a],
  "frame": [r, g, b, a],
  "selection": [r, g, b, a],
  "spectrum_main": [r, g, b, a],
  "spectrum_secondary": [r, g, b, a],
  "spectrum_ref_line": [r, g, b, a]
}
```

Color values are RGBA arrays with values from 0-255.
