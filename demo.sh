#!/bin/bash

# Flash AI Demo Script
# Demonstrates the capabilities of Flash AI

echo "🤖 Flash AI Demo - Intelligent Web Scraping Assistant"
echo "=================================================="
echo ""

cd flash-core

echo "📋 1. Checking Flash AI Status..."
./target/debug/flash status
echo ""

echo "🎯 2. Executing Sample Scraping Commands..."
echo ""

echo "   Command: Find universities in Tokyo"
echo "   $ flash execute \"find 5 universities in tokyo with contact info\""
./target/debug/flash execute "find 5 universities in tokyo with contact info"
echo ""

echo "   Command: Business intelligence query"
echo "   $ flash execute \"get top tech companies in silicon valley\""
./target/debug/flash execute "get top tech companies in silicon valley"
echo ""

echo "   Command: Market research"
echo "   $ flash execute \"find restaurants in paris with michelin stars\""
./target/debug/flash execute "find restaurants in paris with michelin stars"
echo ""

echo "📊 3. Checking Flash AI Status After Tasks..."
./target/debug/flash status
echo ""

echo "🔧 4. Available Commands..."
./target/debug/flash --help
echo ""

echo "✨ Demo completed! Flash AI is ready for intelligent web scraping!"
echo ""
echo "📝 Try these commands:"
echo "   • ./target/debug/flash chat                     # Interactive mode"
echo "   • ./target/debug/flash dashboard               # Web interface"
echo "   • ./target/debug/flash proxy --add <proxy>     # Manage proxies"
echo ""
echo "🎯 Natural Language Examples:"
echo "   • \"find 100 university in dhaka, BD target: url, name, location, email\""
echo "   • \"scrape job postings for AI engineers in Silicon Valley\""
echo "   • \"get contact info for marketing agencies in europe\""
echo ""
echo "Happy scraping! 🚀"
