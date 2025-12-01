# Themes System

The nanometers audio visualizer includes a comprehensive theme system that allows users to customize the appearance of the application.

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

## Rainglow Themes

The app now includes **200+ Rainglow themes** from the popular VS Code theme collection. These themes are automatically converted from their original format and made available in the theme selector.

### Rainglow Collection Features
- **Extensive variety**: Over 200 professionally designed themes
- **All variants included**: Regular, light, and contrast versions
- **Automatic conversion**: VS Code themes adapted for nanometers color system
- **Customizable mapping**: Adjust how Rainglow colors map to app elements

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

### Creating Custom Themes
1. Click "New" in the Theme settings panel
2. Enter a name for your custom theme
3. Use the color pickers to customize colors
4. Click "Save" to store the theme

### Editing Themes
- **Built-in themes**: Creates a copy with a new name for editing
- **Rainglow themes**: Protected from editing (use Rainglow Mapping instead)
- **Custom themes**: Edit directly with full color picker interface

### Rainglow Color Mapping
1. Click "Rainglow Mapping" in the Theme settings panel
2. Select a Rainglow theme for preview
3. Map VS Code color properties to nanometers elements:
   - **App Colors**: Main, Background, Text, Accent, etc.
   - **Spectrum Colors**: Frame, Selection, Main/Secondary spectrum lines
4. Click "Save Mapping" to apply changes to all Rainglow themes

### Theme Protection
Protected themes cannot be modified:
- **Built-in themes**: Cannot be deleted or directly overwritten
- **Rainglow themes**: Cannot be edited (adjust mapping instead)
- Editing protected themes creates an editable copy

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

## Rainglow Integration

### Color Mapping System

Rainglow themes use VS Code's color scheme, which is automatically mapped to nanometers colors:

#### Default Mappings
- `main` ← `activityBarBadge.background`
- `bg` ← `editor.background`  
- `bgaccent` ← `sideBar.background`
- `text` ← `editor.foreground`
- `accent` ← `list.activeSelectionBackground`
- `frame` ← `editorIndentGuide.background`
- `selection` ← `editor.selectionBackground`
- `spectrum_main` ← `activityBarBadge.background`
- `spectrum_secondary` ← `statusBar.background`
- `spectrum_ref_line` ← `editorLineNumber.foreground`

### Customizing Mappings

1. Open Theme settings and click "Rainglow Mapping"
2. Choose a theme to preview mapping changes
3. Select different VS Code color properties for each app element
4. Available properties include:
   - `editor.background`, `editor.foreground`
   - `statusBar.background`, `statusBar.foreground` 
   - `activityBar.background`, `sideBar.background`
   - `list.activeSelectionBackground`
   - And 50+ other VS Code color properties

### Theme Naming

Rainglow themes appear with "Rainglow: " prefix to distinguish them from built-in themes.

### Build-Time Integration

Rainglow themes are processed at compile time:
- Only the `colors` section from each theme is included
- Themes are embedded in the binary for offline use
- No runtime dependency on external theme files
