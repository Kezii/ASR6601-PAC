#[doc = "Register `PeriphID2` reader"]
pub type R = crate::R<PeriphId2Spec>;
#[doc = "Field `DESIGNER1` reader - designer 1, fixed 0x0"]
pub type Designer1R = crate::FieldReader;
#[doc = "revision 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revision0 {
    #[doc = "0: r1p0"]
    R1p0 = 0,
    #[doc = "1: r1p1"]
    R1p1 = 1,
    #[doc = "2: r1p3/r1p4"]
    R1p3_4 = 2,
    #[doc = "3: r1p5"]
    R1p5 = 3,
}
impl From<Revision0> for u8 {
    #[inline(always)]
    fn from(variant: Revision0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revision0 {
    type Ux = u8;
}
impl crate::IsEnum for Revision0 {}
#[doc = "Field `REVISION0` reader - revision 0"]
pub type Revision0R = crate::FieldReader<Revision0>;
impl Revision0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revision0> {
        match self.bits {
            0 => Some(Revision0::R1p0),
            1 => Some(Revision0::R1p1),
            2 => Some(Revision0::R1p3_4),
            3 => Some(Revision0::R1p5),
            _ => None,
        }
    }
    #[doc = "r1p0"]
    #[inline(always)]
    pub fn is_r1p0(&self) -> bool {
        *self == Revision0::R1p0
    }
    #[doc = "r1p1"]
    #[inline(always)]
    pub fn is_r1p1(&self) -> bool {
        *self == Revision0::R1p1
    }
    #[doc = "r1p3/r1p4"]
    #[inline(always)]
    pub fn is_r1p3_4(&self) -> bool {
        *self == Revision0::R1p3_4
    }
    #[doc = "r1p5"]
    #[inline(always)]
    pub fn is_r1p5(&self) -> bool {
        *self == Revision0::R1p5
    }
}
impl R {
    #[doc = "Bits 0:3 - designer 1, fixed 0x0"]
    #[inline(always)]
    pub fn designer1(&self) -> Designer1R {
        Designer1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - revision 0"]
    #[inline(always)]
    pub fn revision0(&self) -> Revision0R {
        Revision0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "peripheral ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId2Spec;
impl crate::RegisterSpec for PeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id2::R`](R) reader structure"]
impl crate::Readable for PeriphId2Spec {}
