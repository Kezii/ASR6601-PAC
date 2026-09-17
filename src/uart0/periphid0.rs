#[doc = "Register `PERIPHID0` reader"]
pub type R = crate::R<Periphid0Spec>;
#[doc = "Field `PARTNUMBER0` reader - part number 0, fixed 0x11"]
pub type Partnumber0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - part number 0, fixed 0x11"]
    #[inline(always)]
    pub fn partnumber0(&self) -> Partnumber0R {
        Partnumber0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "peripheral ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid0Spec;
impl crate::RegisterSpec for Periphid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid0::R`](R) reader structure"]
impl crate::Readable for Periphid0Spec {}
