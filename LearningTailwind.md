# Advice List

## Advice 1

[Responsive Design](https://tailwindcss.com/docs/responsive-design)

By default, Tailwind uses a mobile-first breakpoint system, similar to what you might be used to in other frameworks like Bootstrap.

What this means is that unprefixed utilities (like uppercase) take effect on all screen sizes, while prefixed utilities (like `md:uppercase`) only take effect at the specified breakpoint and above.

Where this approach surprises people most often is that to style something for mobile, you need to use the unprefixed version of a utility, not the `sm:` prefixed version. Don’t think of `sm:` as meaning “on small screens”, think of it as “at the small breakpoint“.

### Wrong way to use breakpoints

```html
<!-- This will only center text on screens 640px and wider, not on small screens -->
<div class="sm:text-center"></div>
```

### Correct way to use breakpoints

```html
<!-- This will center text on mobile, and left align it on screens 640px and wider -->
<div class="text-center sm:text-left"></div>
```

**For this reason, it’s often a good idea to implement the mobile layout for a design first, then layer on any changes that make sense for `sm` screens, followed by `md` screens, etc.**
