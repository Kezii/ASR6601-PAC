#[doc = "Register `QSPI_MIR` reader"]
pub type R = crate::R<QspiMirSpec>;
#[doc = "Register `QSPI_MIR` writer"]
pub type W = crate::W<QspiMirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "miss times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_mir::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_mir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiMirSpec;
impl crate::RegisterSpec for QspiMirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_mir::R`](R) reader structure"]
impl crate::Readable for QspiMirSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_mir::W`](W) writer structure"]
impl crate::Writable for QspiMirSpec {
    type Safety = crate::Unsafe;
}
