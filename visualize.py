import json
import html

def get_element_type(element):
    """Extract element type name"""
    if isinstance(element, dict) and len(element) == 1:
        return list(element.keys())[0]
    return "Unknown"

def get_location(element):
    """Extract start, end positions"""
    element_data = list(element.values())[0] if len(element) == 1 else element
    if isinstance(element_data, dict) and 'location' in element_data:
        loc = element_data['location']
        return loc['start'], loc['end']
    return 0, 0

def collect_highlights(elements, highlights, depth=0):
    """Collect all highlights from parse tree"""
    colors = ['#ff6b6b', '#4ecdc4', '#45b7d1', '#96ceb4', '#feca57', '#ff9ff3', '#54a0ff', '#5f27cd']

    for element in elements:
        start, end = get_location(element)
        element_type = get_element_type(element)

        highlights.append({
            'start': start,
            'end': end,
            'type': element_type,
            'color': colors[depth % len(colors)],
            'depth': depth
        })

        # Process child elements
        element_data = list(element.values())[0] if len(element) == 1 else element
        if isinstance(element_data, dict) and 'content' in element_data:
            content = element_data['content']
            if isinstance(content, list):
                collect_highlights(content, highlights, depth + 1)

def get_element_content_preview(element, original_text_bytes):
    """Get a preview of the element's actual text content using byte positions"""
    start, end = get_location(element)
    if start == 0 and end == 0:
        return ""

    # Extract bytes and decode to string
    try:
        content_bytes = original_text_bytes[start:end]
        content = content_bytes.decode('utf-8')
    except (IndexError, UnicodeDecodeError):
        return f"[Error: byte range {start}~{end}]"

    # Truncate long content and escape HTML
    if len(content) > 50:
        content = content[:47] + "..."

    # Replace newlines with visual indicator
    content = content.replace('\n', '\\n')
    return html.escape(content)

def generate_interactive_tree_html(elements, original_text_bytes, depth=0):
    """Generate interactive collapsible parse tree HTML"""
    html_parts = []
    indent = "&nbsp;&nbsp;" * depth

    for i, element in enumerate(elements):
        element_type = get_element_type(element)
        start, end = get_location(element)
        content_preview = get_element_content_preview(element, original_text_bytes)

        # Generate unique ID for this node
        node_id = f"node_{depth}_{i}"

        # Check if element has children
        element_data = list(element.values())[0] if len(element) == 1 else element
        has_children = (isinstance(element_data, dict) and
                       'content' in element_data and
                       isinstance(element_data['content'], list) and
                       len(element_data['content']) > 0)

        # Create expandable/collapsible node
        if has_children:
            html_parts.append(f"""
                <div class="tree-node">
                    {indent}<span class="tree-toggle" onclick="toggleNode('{node_id}')">▼</span>
                    <span class="tree-element">{element_type}</span>
                    <span class="tree-position">({start}~{end})</span>
                    {f'<span class="tree-content">"{content_preview}"</span>' if content_preview else ''}
                </div>
                <div id="{node_id}" class="tree-children">
                    {generate_interactive_tree_html(element_data['content'], original_text_bytes, depth + 1)}
                </div>
            """)
        else:
            # Leaf node
            html_parts.append(f"""
                <div class="tree-node">
                    {indent}<span class="leaf-icon">├─</span>
                    <span class="tree-element">{element_type}</span>
                    <span class="tree-position">({start}~{end})</span>
                    {f'<span class="tree-content">"{content_preview}"</span>' if content_preview else ''}
                </div>
            """)

    return ''.join(html_parts)

def byte_to_char_index(text_bytes, byte_index):
    """Convert byte index to character index"""
    try:
        return len(text_bytes[:byte_index].decode('utf-8'))
    except UnicodeDecodeError:
        # Fallback: try to find the closest valid position
        for i in range(byte_index, -1, -1):
            try:
                return len(text_bytes[:i].decode('utf-8'))
            except UnicodeDecodeError:
                continue
        return 0

def main():
    # Load files
    with open('ParseResult.json', 'r', encoding='utf-8') as f:
        parse_result = json.load(f)

    with open('ToParse.txt', 'r', encoding='utf-8') as f:
        original_text = f.read()

    # Convert text to bytes for index conversion
    text_bytes = original_text.encode('utf-8')

    # Collect highlights
    highlights = []
    collect_highlights(parse_result, highlights)

    # Convert byte indices to character indices
    # Remove debug prints - we don't need highlights anymore
    pass

    # Sort by position
    highlights.sort(key=lambda x: (x['start'], -x['depth']))

    # Generate HTML
    html_output = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>SevenMark Parse Visualization</title>
    <style>
        :root {{
            --bg-primary: #1e1e1e;
            --bg-secondary: #2d2d30;
            --bg-tertiary: #383838;
            --text-primary: #ffffff;
            --text-secondary: #d4d4d4;
            --accent: #569cd6;
            --accent-hover: #4b8bbf;
            --border: rgba(255,255,255,0.2);
            --shadow: 0 4px 16px rgba(0,0,0,0.3);
            --radius: 8px;
            --vs-blue: #569cd6;
            --vs-orange: #ce9178;
            --vs-green: #6a9955;
            --vs-purple: #c586c0;
            --vs-yellow: #dcdcaa;
            --vs-red: #f44747;
        }}

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', system-ui, sans-serif;
            background: var(--bg-primary);
            color: var(--text-primary);
            line-height: 1.6;
            min-height: 100vh;
        }}

        .container {{
            max-width: 1600px;
            margin: 0 auto;
            padding: 32px 40px;
            width: 95%;
        }}

        .header {{
            text-align: center;
            margin-bottom: 48px;
        }}

        h1 {{
            font-size: 2.2rem;
            font-weight: 600;
            color: var(--vs-blue);
            margin-bottom: 12px;
            font-family: 'JetBrains Mono', monospace;
        }}

        .subtitle {{
            color: var(--text-secondary);
            font-size: 1.1rem;
            font-weight: 400;
        }}

        .section {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            margin-bottom: 16px;
            overflow: hidden;
            box-shadow: var(--shadow);
        }}

        .section-header {{
            padding: 16px 20px;
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            user-select: none;
            transition: background-color 0.2s ease;
            background: rgba(0,0,0,0.2);
        }}

        .section-header:hover {{
            background: rgba(86,156,214,0.15);
        }}

        .section-title {{
            font-size: 1.3rem;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 12px;
        }}

        .section-toggle {{
            font-size: 1.2rem;
            color: var(--text-secondary);
            transition: transform 0.3s ease, color 0.2s ease;
        }}

        .section-toggle:hover {{
            color: var(--accent);
        }}

        .section-content {{
            transition: max-height 0.3s ease, opacity 0.3s ease;
            overflow: hidden;
        }}

        .section-content.collapsed {{
            max-height: 0;
            opacity: 0;
        }}

        .original-text {{
            background: var(--bg-tertiary);
            padding: 20px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 14px;
            line-height: 1.6;
            white-space: pre-wrap;
            overflow-x: auto;
            color: var(--text-primary);
        }}

        .tree-container {{
            padding: 20px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 14px;
            background: var(--bg-tertiary);
            color: var(--text-primary);
        }}

        .tree-controls {{
            display: flex;
            gap: 12px;
            margin-bottom: 20px;
        }}

        .btn {{
            background: var(--bg-primary);
            border: 1px solid var(--border);
            color: var(--text-primary);
            padding: 6px 12px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: normal;
            cursor: pointer;
            transition: background-color 0.2s ease;
            font-family: inherit;
        }}

        .btn:hover {{
            background: var(--accent);
            color: var(--bg-primary);
        }}

        .tree-node {{
            margin: 3px 0;
            padding: 4px 6px;
            font-size: 14px;
            border-radius: 4px;
            transition: background-color 0.2s ease;
        }}

        .tree-node:hover {{
            background: rgba(86,156,214,0.1);
        }}

        .tree-toggle {{
            display: inline-block;
            width: 18px;
            text-align: center;
            color: var(--vs-blue);
            cursor: pointer;
            user-select: none;
            transition: all 0.2s ease;
            font-weight: bold;
            font-size: 14px;
        }}

        .tree-toggle:hover {{
            color: var(--vs-purple);
            transform: scale(1.1);
        }}

        .tree-element {{
            color: var(--vs-orange);
            font-weight: bold;
        }}

        .tree-position {{
            color: var(--text-secondary);
            font-size: 12px;
        }}

        .tree-content {{
            color: var(--vs-yellow);
            font-style: italic;
            margin-left: 8px;
        }}

        .tree-children {{
            margin-left: 16px;
            border-left: 1px solid var(--border);
            padding-left: 12px;
        }}

        .tree-children.collapsed {{
            display: none;
        }}

        .leaf-icon {{
            color: var(--vs-green);
            margin-right: 4px;
        }}

        @media (max-width: 768px) {{
            .container {{
                padding: 20px 16px;
            }}

            h1 {{
                font-size: 2rem;
            }}

            .section-header {{
                padding: 16px 20px;
            }}

            .original-text, .tree-container {{
                padding: 20px;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>SevenMark Parse Visualization</h1>
            <p class="subtitle">Interactive parser analysis with collapsible sections</p>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('original-section')">
                <div class="section-title">
                    <span>📝</span>
                    <span>Original Text</span>
                </div>
                <div class="section-toggle" id="original-toggle">▶</div>
            </div>
            <div class="section-content collapsed" id="original-section">
                <div class="original-text">
{html.escape(original_text)}
                </div>
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('tree-section')">
                <div class="section-title">
                    <span>🌳</span>
                    <span>Interactive Parse Tree</span>
                </div>
                <div class="section-toggle" id="tree-toggle">▼</div>
            </div>
            <div class="section-content" id="tree-section">
                <div class="tree-container">
                    <div class="tree-controls">
                        <button class="btn" onclick="expandAll()">Expand All</button>
                        <button class="btn" onclick="collapseAll()">Collapse All</button>
                    </div>
                    {generate_interactive_tree_html(parse_result, text_bytes)}
                </div>
            </div>
        </div>
    </div>

    <script>
        function toggleSection(sectionId) {{
            const section = document.getElementById(sectionId);
            const toggle = document.getElementById(sectionId.replace('-section', '-toggle'));

            if (section.classList.contains('collapsed')) {{
                section.classList.remove('collapsed');
                toggle.textContent = '▼';
                toggle.style.transform = 'rotate(0deg)';
            }} else {{
                section.classList.add('collapsed');
                toggle.textContent = '▶';
                toggle.style.transform = 'rotate(-90deg)';
            }}
        }}

        function toggleNode(nodeId) {{
            const element = document.getElementById(nodeId);
            const toggle = element.previousElementSibling.querySelector('.tree-toggle');

            if (element.classList.contains('collapsed')) {{
                element.classList.remove('collapsed');
                toggle.textContent = '▼';
            }} else {{
                element.classList.add('collapsed');
                toggle.textContent = '▶';
            }}
        }}

        function expandAll() {{
            document.querySelectorAll('.tree-children.collapsed').forEach(el => {{
                el.classList.remove('collapsed');
            }});
            document.querySelectorAll('.tree-toggle').forEach(toggle => {{
                toggle.textContent = '▼';
            }});
        }}

        function collapseAll() {{
            document.querySelectorAll('.tree-children').forEach(el => {{
                el.classList.add('collapsed');
            }});
            document.querySelectorAll('.tree-toggle').forEach(toggle => {{
                toggle.textContent = '▶';
            }});
        }}
    </script>
</body>
</html>
    """

    with open('visualization.html', 'w', encoding='utf-8') as f:
        f.write(html_output)

    print("Done! visualization.html created")
    print("Open in browser to view results")

if __name__ == "__main__":
    main()