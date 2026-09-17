#[doc = "Register `SAESR` reader"]
pub type R = crate::R<SaesrSpec>;
#[doc = "Register `SAESR` writer"]
pub type W = crate::W<SaesrSpec>;
#[doc = "Field `CRYPTO_DONE` reader - Crypto done"]
pub type CryptoDoneR = crate::BitReader;
#[doc = "Field `CRYPTO_DONE` writer - Crypto done"]
pub type CryptoDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLUE_ERROR` reader - Glue error"]
pub type GlueErrorR = crate::BitReader;
#[doc = "Field `GLUE_ERROR` writer - Glue error"]
pub type GlueErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_ERROR` reader - Command error"]
pub type CommandErrorR = crate::BitReader;
#[doc = "Field `COMMAND_ERROR` writer - Command error"]
pub type CommandErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGORITHM_RUNNING` reader - Algorithm running"]
pub type AlgorithmRunningR = crate::BitReader;
#[doc = "Field `ALGORITHM_RUNNING` writer - Algorithm running"]
pub type AlgorithmRunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLOW_BUSY` reader - Flow busy"]
pub type FlowBusyR = crate::BitReader;
#[doc = "Field `FLOW_BUSY` writer - Flow busy"]
pub type FlowBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Crypto done"]
    #[inline(always)]
    pub fn crypto_done(&self) -> CryptoDoneR {
        CryptoDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Glue error"]
    #[inline(always)]
    pub fn glue_error(&self) -> GlueErrorR {
        GlueErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command error"]
    #[inline(always)]
    pub fn command_error(&self) -> CommandErrorR {
        CommandErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Algorithm running"]
    #[inline(always)]
    pub fn algorithm_running(&self) -> AlgorithmRunningR {
        AlgorithmRunningR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flow busy"]
    #[inline(always)]
    pub fn flow_busy(&self) -> FlowBusyR {
        FlowBusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crypto done"]
    #[inline(always)]
    pub fn crypto_done(&mut self) -> CryptoDoneW<'_, SaesrSpec> {
        CryptoDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Glue error"]
    #[inline(always)]
    pub fn glue_error(&mut self) -> GlueErrorW<'_, SaesrSpec> {
        GlueErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Command error"]
    #[inline(always)]
    pub fn command_error(&mut self) -> CommandErrorW<'_, SaesrSpec> {
        CommandErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Algorithm running"]
    #[inline(always)]
    pub fn algorithm_running(&mut self) -> AlgorithmRunningW<'_, SaesrSpec> {
        AlgorithmRunningW::new(self, 3)
    }
    #[doc = "Bit 4 - Flow busy"]
    #[inline(always)]
    pub fn flow_busy(&mut self) -> FlowBusyW<'_, SaesrSpec> {
        FlowBusyW::new(self, 4)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`saesr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaesrSpec;
impl crate::RegisterSpec for SaesrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saesr::R`](R) reader structure"]
impl crate::Readable for SaesrSpec {}
#[doc = "`write(|w| ..)` method takes [`saesr::W`](W) writer structure"]
impl crate::Writable for SaesrSpec {
    type Safety = crate::Unsafe;
}
