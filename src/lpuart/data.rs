#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `LPUART_DATA` reader - Lpuart data"]
pub type LpuartDataR = crate::FieldReader;
#[doc = "Field `LPUART_DATA` writer - Lpuart data"]
pub type LpuartDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Lpuart data"]
    #[inline(always)]
    pub fn lpuart_data(&self) -> LpuartDataR {
        LpuartDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lpuart data"]
    #[inline(always)]
    pub fn lpuart_data(&mut self) -> LpuartDataW<'_, DataSpec> {
        LpuartDataW::new(self, 0)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
