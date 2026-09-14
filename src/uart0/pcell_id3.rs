#[doc = "Register `PCellID3` reader"]
pub type R = crate::R<PcellId3Spec>;
#[doc = "Field `CellID3` reader - primecell ID 3, fixed 0xb1"]
pub type CellId3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - primecell ID 3, fixed 0xb1"]
    #[inline(always)]
    pub fn cell_id3(&self) -> CellId3R {
        CellId3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "primecell ID register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId3Spec;
impl crate::RegisterSpec for PcellId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id3::R`](R) reader structure"]
impl crate::Readable for PcellId3Spec {}
