#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CLK_32M_RDY_BAT_SR` reader - clk 32m ready status"]
pub type Clk32mRdyBatSrR = crate::BitReader;
#[doc = "Field `IRQ_DIG_SR` reader - irq dig status flag"]
pub type IrqDigSrR = crate::FieldReader;
#[doc = "Field `BUSY_DIG_SR` reader - busy dig status flag"]
pub type BusyDigSrR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - clk 32m ready status"]
    #[inline(always)]
    pub fn clk_32m_rdy_bat_sr(&self) -> Clk32mRdyBatSrR {
        Clk32mRdyBatSrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:7 - irq dig status flag"]
    #[inline(always)]
    pub fn irq_dig_sr(&self) -> IrqDigSrR {
        IrqDigSrR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - busy dig status flag"]
    #[inline(always)]
    pub fn busy_dig_sr(&self) -> BusyDigSrR {
        BusyDigSrR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
