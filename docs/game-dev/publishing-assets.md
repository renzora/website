# Publishing Assets

Sell or share your creations on the Renzora Marketplace.

## Creator onboarding

Before publishing, complete the creator setup:

1. Go to [Marketplace → Sell](/marketplace/sell)
2. Fill in your creator profile (display name, bio, links)
3. For paid assets: add payout information (PayPal or bank transfer)
4. Accept the Marketplace Creator Agreement

## Preparing your asset

### File structure

Organize your asset in a clean folder:

```
my_asset/
├── README.md              # Usage instructions
├── assets/
│   ├── models/            # 3D models
│   ├── textures/          # Texture files
│   ├── materials/         # Material files
│   └── scripts/           # Script files
└── examples/
    └── demo_scene.ron     # Example scene showing the asset in use
```

### Checklist

- All file paths are relative (no absolute paths)
- No project-specific configuration files
- Include a README with setup instructions
- Test in a clean project to ensure nothing is missing
- Remove any temporary or debug files

## Creating a listing

Go to [Marketplace → Upload](/marketplace/upload) and fill in:

| Field | Description |
|-------|-------------|
| **Title** | Clear, descriptive name (e.g., "Low-Poly Fantasy Weapons Pack") |
| **Description** | What's included, features, usage instructions |
| **Category** | Scripts, Models, Textures, Audio, etc. |
| **Tags** | Up to 10 keywords for discoverability |
| **Thumbnail** | 800×450 image shown in search results |
| **Screenshots** | Up to 6 images showing the asset in use |
| **Price** | Free or amount in credits |
| **Asset file** | ZIP of your asset folder |

### Writing a good description

- Lead with what the asset does
- List what's included (file count, formats)
- Specify engine version compatibility
- Include setup instructions
- Mention any dependencies

## Pricing

- **Free** — great for building reputation and community goodwill
- **Paid** — set a price in credits (1 credit ≈ $0.01 USD)
- Look at similar assets to price competitively
- You can change the price at any time

## Review process

After submission:

1. Your asset enters a review queue
2. Renzora staff checks for: malware, copyright issues, quality standards
3. Typically reviewed within 24–48 hours
4. You'll be notified of approval or feedback for changes

## Updating published assets

1. Go to your asset's page → **Edit**
2. Upload a new version (ZIP)
3. Add a changelog entry describing what changed
4. Users are notified of the update

Version history is preserved — users can always access previous versions.

## Analytics

View your asset performance on the **Creator Dashboard**:

- Downloads / purchases over time
- Revenue earned
- Ratings and reviews
- Geographic distribution of users

## Best practices

- **Quality screenshots** — show the asset in a polished scene, not a gray void
- **Keep it updated** — fix bugs, add compatibility with new engine versions
- **Respond to reviews** — engage with your community
- **Create a demo scene** — let users try before they buy
- **Bundle related assets** — packs sell better than individual items
