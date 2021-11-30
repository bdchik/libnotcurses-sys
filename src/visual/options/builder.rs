//!

use crate::{
    NcAlign, NcBlitter, NcBlitterApi, NcDim, NcOffset, NcPlane, NcRgba, NcScale, NcVisualOptions,
};

/// Builder object for [`NcVisualOptions`].
///
/// Can be constructed by calling [`NcVisualOptions::builder()`].
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
#[derive(Debug, Default)]
pub struct NcVisualOptionsBuilder<'ncplane> {
    plane: Option<&'ncplane mut NcPlane>,
    scale: NcScale,
    y: NcOffset,
    x: NcOffset,
    region_yx_lenyx: Option<(NcDim, NcDim, NcDim, NcDim)>,
    cell_offset_yx: Option<(NcDim, NcDim)>,
    blitter: NcBlitter,
    flags: u32,
    transcolor: NcRgba,
}

impl<'ncplane> NcVisualOptionsBuilder<'ncplane> {
    /// Sets the `NcPlane` where the blitting will be done.
    ///
    /// This `NcPlane` could also be considered the parent of a new plane where
    /// the blitting will occur by utilizing the [`child`] method.
    ///
    /// When no `NcPlane` is provided, one will be created using the exact size
    /// necessary to render the source with perfect fidelity (this might be
    /// smaller or larger than the rendering area).
    ///
    /// Default: *none* (no plane).
    ///
    /// See also: *[`parent`]*, *[`child`]*, *[`no_plane`]*.
    ///
    /// [`child`]: NcVisualOptionsBuilder#method.child
    /// [`parent`]: NcVisualOptionsBuilder#method.parent
    /// [`no_plane`]: NcVisualOptionsBuilder#method.no_plane
    pub fn plane(mut self, plane: &'ncplane mut NcPlane) -> Self {
        self.plane = Some(plane);
        self
    }

    /// If true, a [`plane`] must also be provided, which will be the parent
    /// of a new child `NcPlane` into which the blitting will be done.
    ///
    /// If false, the blitting will occur in the provided [`plane`], if any,
    /// or in a newly created `NcPlane` otherwise.
    ///
    /// Default: *false* (no child plaane).
    ///
    /// Effect: Sets the [`CHILDPLANE`] flag.
    ///
    /// See also: *[`plane`]*, *[`parent`]*.
    ///
    /// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
    /// [`plane`]: NcVisualOptionsBuilder#method.plane
    /// [`parent`]: NcVisualOptionsBuilder#method.parent
    pub fn child(mut self, child: bool) -> Self {
        if child {
            self.flags |= NcVisualOptions::CHILDPLANE;
        } else {
            self.flags &= !NcVisualOptions::CHILDPLANE;
        }
        self
    }

    /// Sets the `NcPlane` that will be the parent of a new `NcPlane` where
    /// the blitting will be done.
    ///
    /// This is the same as calling both [`plane`] and [`child`].
    ///
    /// See also: *[`plane`]*, *[`child`]*.
    ///
    /// [`plane`]: NcVisualOptionsBuilder#method.plane
    /// [`child`]: NcVisualOptionsBuilder#method.child
    pub fn parent(mut self, plane: &'ncplane mut NcPlane) -> Self {
        self.plane = Some(plane);
        self.flags |= NcVisualOptions::CHILDPLANE;
        self
    }

    /// Unsets the `NcPlane`.
    ///
    /// Effect: unsets the plane & the [`CHILDPLANE`] flag.
    ///
    /// Default: yes.
    ///
    /// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
    pub fn no_plane(mut self) -> Self {
        self.plane = None;
        self.flags &= !NcVisualOptions::CHILDPLANE;
        self
    }

    /// Sets the `NcScale`.
    ///
    /// Default: *[`NcScale::NOSCALE`][crate::NcScale#associatedconstant.NOSCALE]*.
    pub fn scale(mut self, scale: NcScale) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the vertical placement.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *y* coordinate, and unsets the [`VERALIGNED`] flag.
    ///
    /// [`VERALIGNED`]: NcVisualOptions#associatedconstant.VERALIGNED
    pub fn y(mut self, y: NcOffset) -> Self {
        self.y = y;
        self.flags &= !NcVisualOptions::VERALIGNED;
        self
    }

    /// Sets the horizontal placement.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *x* coordinate, and unsets the [`HORALIGNED`] flag.
    ///
    /// [`HORALIGNED`]: NcVisualOptions#associatedconstant.HORALIGNED
    pub fn x(mut self, x: NcOffset) -> Self {
        self.x = x;
        self.flags &= !NcVisualOptions::HORALIGNED;
        self
    }

    /// Sets the vertical & horizontal placement.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: Sets the *`y` & `x`* coordinates and unsets the [`VERALIGNED`]
    /// & [`HORALIGNED`] flags.
    ///
    /// [`VERALIGNED`]: NcVisualOptions#associatedconstant.VERALIGNED
    /// [`HORALIGNED`]: NcVisualOptions#associatedconstant.HORALIGNED
    pub fn yx(mut self, y: NcOffset, x: NcOffset) -> Self {
        self.y = y;
        self.x = x;
        self.flags &= !NcVisualOptions::VERALIGNED;
        self.flags &= !NcVisualOptions::HORALIGNED;
        self
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`NcAlign::TOP`]*.
    ///
    /// Effect: Sets the *y* alignment and the [`VERALIGNED`] flag.
    ///
    /// [`NcAlign::TOP`]: crate::NcAlign#associatedconstant.TOP
    /// [`VERALIGNED`]: NcVisualOptions#associatedconstant.VERALIGNED
    pub fn valign(mut self, valign: NcAlign) -> Self {
        self.y = valign as NcOffset;
        self.flags |= NcVisualOptions::VERALIGNED;
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`NcAlign::LEFT`]*.
    ///
    /// Effect: Sets the *`x`* alignment and the [`VERALIGNED`] flag.
    ///
    /// [`NcAlign::LEFT`]: crate::NcAlign#associatedconstant.TOP
    /// [`VERALIGNED`]: NcVisualOptions#associatedconstant.VERALIGNED
    pub fn halign(mut self, halign: NcAlign) -> Self {
        self.x = halign as NcOffset;
        self.flags |= NcVisualOptions::HORALIGNED;
        self
    }

    /// Sets the vertical & horizontal alignments.
    ///
    /// Default: *`(`[`NcAlign::TOP`]*`, `*[`NcAlign::LEFT`]`)`*.
    ///
    /// Effect: Sets the *`y` & `x`* alignments and the [`VERALIGNED`] flag.
    ///
    /// [`NcAlign::TOP`]: crate::NcAlign#associatedconstant.TOP
    /// [`NcAlign::LEFT`]: crate::NcAlign#associatedconstant.LEFT
    /// [`VERALIGNED`]: NcVisualOptions#associatedconstant.VERALIGNED
    pub fn align(mut self, valign: NcAlign, halign: NcAlign) -> Self {
        self.y = valign as NcOffset;
        self.x = halign as NcOffset;
        self.flags |= NcVisualOptions::VERALIGNED;
        self.flags |= NcVisualOptions::HORALIGNED;
        self
    }

    /// Choose the `NcBlitter`.
    ///
    /// Default: *[`NcBlitter::DEFAULT`]*.
    ///
    /// [`NcBlitter::DEFAULT`]: crate::NcBlitter#associatedconstant.DEFAULT
    pub fn blitter(mut self, blitter: NcBlitter) -> Self {
        self.blitter = blitter;
        self
    }

    /// Choose `NcBlitter::PIXEL` for the blitter.
    ///
    /// [`NcBlitter::PIXEL`]: crate::NcBlitter#associatedconstant.PIXEL
    pub fn pixel(mut self) -> Self {
        self.blitter = NcBlitter::PIXEL;
        self
    }

    /// Choose the color to be considered transparent, or `None`.
    ///
    /// Default: *none*.
    ///
    /// Efect: (Un)Sets the transparent color, and the [`ADDALPHA`] flag.
    ///
    /// [`ADDALPHA`]: NcVisualOptions#associatedconstant.ADDALPHA
    pub fn transcolor(mut self, color: Option<NcRgba>) -> Self {
        if let Some(color) = color {
            self.transcolor = color;
            self.flags |= NcVisualOptions::ADDALPHA;
        } else {
            self.flags &= !NcVisualOptions::ADDALPHA;
        }
        self
    }

    /// Choose whether to use [`NcAlpha::BLEND`] with the [`NcVisual`], so that
    /// the foreground or background colors can be a composite between
    /// a color and the corresponding colors underneath it.
    ///
    /// Default: *false* (no blend).
    ///
    /// Effect: Sets the [`BLEND`] flag.
    ///
    /// [`BLEND`]: NcVisualOptions#associatedconstant.BLEND
    /// [`NcAlpha::Blend`]: crate::NcAlpha#associatedconstant.BLEND
    /// [`NcVisual`]: crate::NcVisual
    pub fn blend(mut self, blend: bool) -> Self {
        if blend {
            self.flags |= NcVisualOptions::BLEND;
        } else {
            self.flags &= !NcVisualOptions::BLEND;
        }
        self
    }

    /// Choose between gracefully degrading the blitter, or fail if the choosen
    /// `NcBlitter` is not supported by the terminal.
    ///
    /// Default: *true* (degrades).
    ///
    /// Effect: Sets the [`NODEGRADE`] flag.
    ///
    /// See also: the [*rules of degradation*].
    ///
    /// [`NODEGRADE`]: NcVisualOptions#associatedconstant.NODEGRADE
    /// [*rules of degradation*]: NcBlitter#degradation
    pub fn degrade(mut self, degrade: bool) -> Self {
        if degrade {
            self.flags &= !NcVisualOptions::NODEGRADE;
        } else {
            self.flags |= NcVisualOptions::NODEGRADE;
        }
        self
    }

    /// Sets the `NOINTERPOLATE` flag.
    ///
    /// Default: *true* (interpolates).
    ///
    pub fn interpolate(mut self, interpolate: bool) -> Self {
        if interpolate {
            self.flags &= !NcVisualOptions::NOINTERPOLATE;
        } else {
            self.flags |= NcVisualOptions::NOINTERPOLATE;
        }
        self
    }

    /// Sets the region to be rendered.
    ///
    /// (start_y, start_x, len_y, len_x)
    pub fn region(mut self, beg_y: NcDim, beg_x: NcDim, len_y: NcDim, len_x: NcDim) -> Self {
        self.region_yx_lenyx = Some((beg_y, beg_x, len_y, len_x));
        self
    }

    /// Sets the pixel offset within the [`NcCell`][crate::NcCell].
    ///
    ///
    pub fn cell_offset(mut self, y: NcDim, x: NcDim) -> Self {
        self.cell_offset_yx = Some((y, x));
        self
    }

    /// Finishes the building and returns [`NcVisualOptions`].
    pub fn build(self) -> NcVisualOptions {
        NcVisualOptions::new(
            self.plane,
            self.scale,
            self.y,
            self.x,
            self.region_yx_lenyx,
            self.cell_offset_yx,
            self.blitter,
            self.flags,
            self.transcolor,
        )
    }
}
