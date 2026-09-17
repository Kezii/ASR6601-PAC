#[doc = "Register `PCELLID3` reader"]
pub type R = crate::R<Pcellid3Spec>;
#[doc = "Field `CellID3` reader - primecell ID 3, fixed 0xb1"]
pub type CellId3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - primecell ID 3, fixed 0xb1"]
    #[inline(always)]
    pub fn cell_id3(&self) -> CellId3R {
        CellId3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "primecell ID register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcellid3Spec;
impl crate::RegisterSpec for Pcellid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcellid3::R`](R) reader structure"]
impl crate::Readable for Pcellid3Spec {}
