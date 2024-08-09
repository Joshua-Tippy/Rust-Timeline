# Timeline Command Line Utility Requirements

## Overview

This document outlines the requirements for a command line utility designed to generate a timeline SVG file from a YAML input file. The utility will provide customizable options for generating the timeline based on user-defined parameters and historical data.

## Command Line Utility

### Basic Command

The basic command to run the utility is:

```sh
$ timeline history.yaml image.svg
```

### Arguments

#### 1. Start Date and End Date

- **Description:** Specifies the date range for generating the timeline.
- **Format:** Dates can be provided in the following formats:
  - YYYY
  - MM-YYYY
  - DD-MM-YYYY
  - DD-MM-YYYYzHH-MM-SS (full ISO format)

#### 2. Tags

- **Description:** A list of tags to filter events to include in the timeline. If not provided, all events will be included.
- **Format:** Comma-separated list of tags.

#### 3. Timeline Direction

- **Description:** Specifies the direction of the timeline.
- **Options:**
  - `bottom-up`
  - `top-down`

#### 4. Font/Size (Optional)

- **Description:** The font type and size for the text in the timeline.
- **Format:** String specifying the font and size.

#### 5. Tag Colors (Optional)

- **Description:** Custom colors for each tag.
- **Format:** Key-value pairs of tags and their corresponding colors.

### Event Order

- **Description:** When multiple events have the same date, they will be ordered based on their appearance in the YAML file.

### Event ID

- **Description:** A unique identifier for each event.
- **Format:** Alphanumeric string without whitespace.

### Crates to use
structopt
flexi_logger

plus others you need








