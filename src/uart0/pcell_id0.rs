#[doc = "Register `PCellID0` reader"]
pub type R = crate::R<PcellId0Spec>;
#[doc = "Field `CellID0` reader - primecell ID 0, fixed 0x0d"]
pub type CellId0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - primecell ID 0, fixed 0x0d"]
    #[inline(always)]
    pub fn cell_id0(&self) -> CellId0R {
        CellId0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "primecell ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcell_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId0Spec;
impl crate::RegisterSpec for PcellId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id0::R`](R) reader structure"]
impl crate::Readable for PcellId0Spec {}
