#[doc = "Register `PCellID1` reader"]
pub type R = crate::R<PcellId1Spec>;
#[doc = "Field `CellID1` reader - primecell ID 1, fixed 0xf0"]
pub type CellId1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - primecell ID 1, fixed 0xf0"]
    #[inline(always)]
    pub fn cell_id1(&self) -> CellId1R {
        CellId1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "primecell ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId1Spec;
impl crate::RegisterSpec for PcellId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id1::R`](R) reader structure"]
impl crate::Readable for PcellId1Spec {}
