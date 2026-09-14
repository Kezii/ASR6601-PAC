#[doc = "Register `PeriphID1` reader"]
pub type R = crate::R<PeriphId1Spec>;
#[doc = "Field `PARTNUMBER1` reader - part number 1, fixed 0x0"]
pub type Partnumber1R = crate::FieldReader;
#[doc = "Field `DESIGNER0` reader - designer 0, fixed 0x1"]
pub type Designer0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - part number 1, fixed 0x0"]
    #[inline(always)]
    pub fn partnumber1(&self) -> Partnumber1R {
        Partnumber1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - designer 0, fixed 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> Designer0R {
        Designer0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "peripheral ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId1Spec;
impl crate::RegisterSpec for PeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id1::R`](R) reader structure"]
impl crate::Readable for PeriphId1Spec {}
