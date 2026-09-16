#[doc = "Register `QSPI_DR` reader"]
pub type R = crate::R<QspiDrSpec>;
#[doc = "Register `QSPI_DR` writer"]
pub type W = crate::W<QspiDrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiDrSpec;
impl crate::RegisterSpec for QspiDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_dr::R`](R) reader structure"]
impl crate::Readable for QspiDrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_dr::W`](W) writer structure"]
impl crate::Writable for QspiDrSpec {
    type Safety = crate::Unsafe;
}
