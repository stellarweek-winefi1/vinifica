# VineFi - Wine Investment Platform

A modern web application for investing in premium wines built with Next.js, TypeScript, and Tailwind CSS.

## Features

- 🍷 **Landing Page**: Hero section with key statistics and value propositions
- 📈 **Marketplace**: Browse and invest in premium wines from Chile, Argentina, and USA
- 💼 **Portfolio**: Track your wine investments and returns
- 🔄 **Tokenization**: Digitalize your wine collection backed by Stellar blockchain
- ⚡ **Fast Transactions**: 3-5 second settlement times
- 🎨 **Responsive Design**: Optimized for all screen sizes
- ♿ **Accessible**: WCAG 2.2 AA compliant

## Tech Stack

- **Framework**: Next.js 15 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Icons**: Lucide React
- **Animations**: Framer Motion
- **Blockchain**: Stellar (mentioned in copy)

## Getting Started

### Prerequisites

- Node.js 18+ installed
- npm or yarn package manager

### Installation

1. Install dependencies:
   ```bash
   npm install
   ```

2. Run the development server:
   ```bash
   npm run dev
   ```

3. Open [http://localhost:3000](http://localhost:3000) in your browser

### Build for Production

```bash
npm run build
npm start
```

## Project Structure

```
vinefi/
├── app/
│   ├── page.tsx              # Landing page
│   ├── mercado/
│   │   └── page.tsx          # Marketplace
│   ├── portafolio/
│   │   └── page.tsx          # Portfolio
│   ├── digitalizar/
│   │   └── page.tsx          # Tokenization form
│   ├── layout.tsx            # Root layout
│   └── globals.css           # Global styles
├── components/
│   ├── Navigation.tsx        # Main navigation
│   ├── HeroSection.tsx       # Landing hero
│   ├── HowItWorksSection.tsx # Features section
│   ├── CTASection.tsx        # Call-to-action
│   └── WineCard.tsx          # Wine product card
├── lib/
│   └── utils.ts              # Utility functions
└── public/                   # Static assets
```

## Pages

- **/** - Landing page with hero, features, and CTA
- **/mercado** - Browse available wines for investment
- **/portafolio** - View your wine holdings and performance
- **/digitalizar** - Multi-step form to tokenize wine collections

## Accessibility

This project follows WCAG 2.2 AA guidelines:
- Semantic HTML elements
- Proper ARIA labels and roles
- Keyboard navigation support
- Minimum color contrast ratios
- Screen reader friendly

## License

This project is for demonstration purposes.
