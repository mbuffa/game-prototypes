@echo off
git checkout 01-turret-assets
git checkout 01-turret
git checkout 01-turret-assets -- assets/
git restore --staged assets