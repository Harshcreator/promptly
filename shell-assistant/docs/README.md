# Documentation

This directory contains documentation for Shell Assistant.

## Structure

```
docs/
├── enterprise/          # Enterprise deployment documentation
│   └── README.md       # Enterprise deployment guide
└── dev/                # Development documentation (not committed to git)
    ├── IMPLEMENTATION_PROGRESS.md
    ├── IMPLEMENTATION_SUMMARY.md
    ├── QUICK_REFERENCE.md
    └── *.md            # Other development notes
```

## For Users

- **Enterprise Deployment**: See [`enterprise/README.md`](enterprise/README.md)
- **Configuration**: See [`../config.example.yaml`](../config.example.yaml)
- **General Usage**: See main [`README.md`](../README.md)

## For Developers

Development documentation is in `dev/` and is excluded from git (see `.gitignore`).
These docs are for internal development tracking and are not committed to the repository.

If you need development documentation, check:
- Implementation progress in `dev/IMPLEMENTATION_PROGRESS.md`
- Quick reference in `dev/QUICK_REFERENCE.md`
- Test documentation in `dev/test_script.md`
